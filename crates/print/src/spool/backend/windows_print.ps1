# Adapted for NEXOPDF on 2026-10-13; see crates/print/PROVENANCE.md and NOTICE.
# Print a print-ready PDF on Windows (CUPS-independent backend). The job arrives in NEXOPDF_PRINT_* environment
# variables, never as arguments: printer names contain spaces and quotes, and the document's
# path must not be visible in the process list while it prints.
#
# NEXOPDF renders each sheet with its CPU renderer and
# System.Drawing.Printing spools it with the driver's settings — copies, collation, duplex and
# colour reach the spooler as the job's, the way `lp -o` sends them on CUPS. No PDF reader is
# needed, and the calling code touches no Win32 API.
#
# NEXOPDF_PRINT_DRYRUN=1 reads every prepared sheet in memory and prints
# nothing: the tests run the whole rendering path on machines without a printer, CI included.
$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = New-Object System.Text.UTF8Encoding($false)

Add-Type -AssemblyName System.Drawing
# Windows duplex values are binding edges: Vertical = long edge, Horizontal = short edge.
# https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-devmodea
function Get-Duplex($edge) {
    switch ($edge) {
        'long-edge' { [System.Drawing.Printing.Duplex]::Vertical }
        'short-edge' { [System.Drawing.Printing.Duplex]::Horizontal }
        default { [System.Drawing.Printing.Duplex]::Simplex }
    }
}

function Set-JobDuplex($settings, $edge) {
    if ($edge) { $settings.Duplex = Get-Duplex $edge }
}

# Manifest dimensions are DIPs (96/inch), printing dimensions are hundredths of an inch.
# The PDF is already composed: preserve physical size, even if the printer clips its edges.
function Get-SheetGeometry($width, $height) {
    if ([double]::IsNaN($width) -or [double]::IsNaN($height) -or
        [double]::IsInfinity($width) -or [double]::IsInfinity($height) -or
        $width -le 0 -or $height -le 0 -or $width -gt 12288 -or $height -gt 12288) {
        throw 'the PDF sheet dimensions are invalid or exceed 128 inches'
    }
    # DEVMODE paper lengths use signed 16-bit tenths of a millimetre (128 inches fits).
    $drawW = $width * 100.0 / 96.0
    $drawH = $height * 100.0 / 96.0
    [pscustomobject]@{
        Width = $drawW; Height = $drawH; Landscape = ($width -gt $height)
        PaperWidth = [int][Math]::Max(1, [Math]::Round([Math]::Min($drawW, $drawH)))
        PaperHeight = [int][Math]::Max(1, [Math]::Round([Math]::Max($drawW, $drawH)))
    }
}

function Set-SheetSettings($settings, $papers, $sheet, $scaling = 'exact', $fallbackPaper = $null) {
    # Prefer the driver's paper identifier over a custom form when the dimensions match.
    $paper = $papers | Where-Object {
        [Math]::Abs($_.Width - $sheet.PaperWidth) -le 1 -and
        [Math]::Abs($_.Height - $sheet.PaperHeight) -le 1
    } | Select-Object -First 1
    if (-not $paper -and $scaling -eq 'shrink-to-fit') {
        if (-not $fallbackPaper) { throw 'the printer has no default paper for shrink-to-fit' }
        $paper = $fallbackPaper
    }
    if (-not $paper) {
        $paper = [System.Drawing.Printing.PaperSize]::new('PDF sheet', $sheet.PaperWidth, $sheet.PaperHeight)
    }
    $settings.PaperSize = $paper
    $settings.Landscape = $sheet.Landscape
    $settings.Margins = [System.Drawing.Printing.Margins]::new(0, 0, 0, 0)
}

function Get-DrawBox($sheet, $hardMarginX, $hardMarginY) {
    # GDI's origin is the printable area's corner. Offset to the physical paper's corner.
    [System.Drawing.RectangleF]::new(-$hardMarginX, -$hardMarginY, $sheet.Width, $sheet.Height)
}

function Get-ShrinkDrawBox($sheet, $area, $hardMarginX, $hardMarginY) {
    foreach ($value in @($area.X, $area.Y, $area.Width, $area.Height, $hardMarginX, $hardMarginY)) {
        if ([double]::IsNaN($value) -or [double]::IsInfinity($value)) {
            throw 'the printer reported an invalid printable area'
        }
    }
    if ($area.Width -le 0 -or $area.Height -le 0) { throw 'the printer has no printable area' }
    $scale = [Math]::Min(1.0, [Math]::Min($area.Width / $sheet.Width, $area.Height / $sheet.Height))
    $width = $sheet.Width * $scale
    $height = $sheet.Height * $scale
    [System.Drawing.RectangleF]::new(
        $area.X + ($area.Width - $width) / 2.0 - $hardMarginX,
        $area.Y + ($area.Height - $height) / 2.0 - $hardMarginY,
        $width, $height)
}


# Exercise the production settings/placement helpers without an installed printer. The
# caller clears this internal test flag for ordinary submissions.
if ($env:NEXOPDF_PRINT_TEST_SETTINGS -eq '1') {
    $papers = @([System.Drawing.Printing.PaperSize]::new('Driver Letter', 850, 1100))
    $results = foreach ($size in @(@(816, 1056), @(1056, 816), @(793.92, 1122.24), @(816, 1344))) {
        $sheet = Get-SheetGeometry $size[0] $size[1]
        $settings = [System.Drawing.Printing.PageSettings]::new()
        Set-SheetSettings $settings $papers $sheet
        $box = Get-DrawBox $sheet 25 30
        [pscustomobject]@{ Width = $settings.PaperSize.Width; Height = $settings.PaperSize.Height;
            Name = $settings.PaperSize.PaperName; Landscape = $settings.Landscape;
            Margins = @($settings.Margins.Left, $settings.Margins.Right, $settings.Margins.Top, $settings.Margins.Bottom);
            X = $box.X; Y = $box.Y; DrawWidth = $box.Width; DrawHeight = $box.Height
        }
    }
    $invalidRejected = $false
    try { $null = Get-SheetGeometry ([double]::NaN) 100 } catch { $invalidRejected = $true }
    $fallback = [System.Drawing.Printing.PageSettings]::new()
    Set-SheetSettings $fallback $papers (Get-SheetGeometry 1536 1920) 'shrink-to-fit' $papers[0]
    $area = [System.Drawing.RectangleF]::new(25, 30, 800, 1040)
    $shrinkBoxes = foreach ($size in @(@(1536, 1920), @(384, 480))) {
        $box = Get-ShrinkDrawBox (Get-SheetGeometry $size[0] $size[1]) $area 25 30
        ,@($box.X, $box.Y, $box.Width, $box.Height)
    }
    $defaultDuplex = [pscustomobject]@{ Duplex = [System.Drawing.Printing.Duplex]::Vertical }
    Set-JobDuplex $defaultDuplex ''
    $preservedDuplex = $defaultDuplex.Duplex.ToString()
    Set-JobDuplex $defaultDuplex 'off'
    [pscustomobject]@{ Sheets = @($results); LongEdge = (Get-Duplex 'long-edge').ToString();
        ShortEdge = (Get-Duplex 'short-edge').ToString(); Off = (Get-Duplex 'off').ToString();
        DefaultDuplex = $preservedDuplex; ForcedSimplex = $defaultDuplex.Duplex.ToString();
        FallbackPaper = $fallback.PaperSize.PaperName; ShrinkBoxes = @($shrinkBoxes);
        InvalidRejected = $invalidRejected } | ConvertTo-Json -Depth 5 -Compress
    return
}

# NEXOPDF renders with its own CPU renderer. This script only validates the
# private sheet package and talks to the system print queue; no WinRT/GPU PDF path.
$manifest = Get-Content -LiteralPath $env:NEXOPDF_PRINT_FILE -Raw -Encoding UTF8 | ConvertFrom-Json
$sheets = @($manifest.Sheets)
$jobRoot = [System.IO.Path]::GetDirectoryName($env:NEXOPDF_PRINT_FILE)
if ($sheets.Count -lt 1 -or $sheets.Count -gt 1000) { throw 'invalid number of print sheets' }
function Read-Sheet($entry) {
    if ($entry.File -notmatch '^sheet-[0-9]+\.png$') { throw 'invalid print sheet name' }
    $path = [System.IO.Path]::Combine($jobRoot, $entry.File)
    $image = [System.Drawing.Image]::FromFile($path)
    try {
        if ($image.Width -ne $entry.PixelWidth -or $image.Height -ne $entry.PixelHeight) {
            throw 'print sheet dimensions do not match the job'
        }
        return [System.Drawing.Bitmap]::new($image)
    } finally { $image.Dispose() }
}
if ($env:NEXOPDF_PRINT_DRYRUN -eq '1') {
    foreach ($entry in $sheets) {
        $bitmap = Read-Sheet $entry
        $bitmap.Dispose()
    }
    Write-Output ('rendered ' + $sheets.Count + ' sheet(s)')
    return
}
$printer = [System.Drawing.Printing.PrintDocument]::new()
$printer.DocumentName = $env:NEXOPDF_PRINT_TITLE
$settings = $printer.PrinterSettings
if ($env:NEXOPDF_PRINT_PRINTER) {
    try {
        $settings.PrinterName = $env:NEXOPDF_PRINT_PRINTER
    } catch {
        throw ('the printer is not available: ' + $env:NEXOPDF_PRINT_PRINTER)
    }
}
$settings.Copies = [Math]::Min(32767, [Math]::Max(1, [int]$env:NEXOPDF_PRINT_COPIES))
$settings.Collate = ($env:NEXOPDF_PRINT_COLLATE -eq '1')
Set-JobDuplex $settings $env:NEXOPDF_PRINT_DUPLEX
if ($env:NEXOPDF_PRINT_GRAYSCALE -eq '1') { $printer.DefaultPageSettings.Color = $false }
$fallbackPaper = $printer.DefaultPageSettings.PaperSize

# QueryPageSettings runs before each PrintPage, allowing mixed sizes/orientations in one job.
# https://learn.microsoft.com/en-us/dotnet/api/system.drawing.printing.printdocument.querypagesettings
$printer.OriginAtMargins = $false
$printer.PrintController = [System.Drawing.Printing.StandardPrintController]::new()
$script:sheetIndex = 0
$printer.add_QueryPageSettings({
    param($sender, $e)
    $page = $sheets[$script:sheetIndex]
    try {
        $sheet = Get-SheetGeometry $page.WidthDip $page.HeightDip
        Set-SheetSettings $e.PageSettings $sender.PrinterSettings.PaperSizes $sheet $env:NEXOPDF_PRINT_SCALING $fallbackPaper
    } finally { $page = $null }
})
$printer.add_PrintPage({
    param($sender, $e)
    $page = $sheets[$script:sheetIndex]
    $bitmap = $null
    try {
        $sheet = Get-SheetGeometry $page.WidthDip $page.HeightDip
        if ($env:NEXOPDF_PRINT_SCALING -ne 'shrink-to-fit' -and
            ([Math]::Abs($e.PageBounds.Width - $sheet.Width) -gt 2 -or
             [Math]::Abs($e.PageBounds.Height - $sheet.Height) -gt 2)) {
            throw 'the printer cannot use the PDF sheet size; select matching paper and try again'
        }
        $bitmap = Read-Sheet $page
        if ($env:NEXOPDF_PRINT_SCALING -eq 'shrink-to-fit') {
            $box = Get-ShrinkDrawBox $sheet $e.PageSettings.PrintableArea $e.PageSettings.HardMarginX $e.PageSettings.HardMarginY
        } else {
            $box = Get-DrawBox $sheet $e.PageSettings.HardMarginX $e.PageSettings.HardMarginY
        }
        $e.Graphics.DrawImage($bitmap, $box)
    } finally {
        if ($bitmap) { $bitmap.Dispose() }
        $page = $null
    }
    $script:sheetIndex++
    $e.HasMorePages = ($script:sheetIndex -lt $sheets.Count)
})
try {
    $printer.Print()
    Write-Output ('spooled ' + $sheets.Count + ' sheet(s)')
} finally { $printer.Dispose() }
