# Transporte de impressão local

Baseline incorporado de `craft-print` 0.1.0, revisão
`d991bf6792f8ade99c51358bf3c1d902c8cd96b5` de
https://github.com/storytold/craft-libs.

Em 2026-10-13 o NEXOPDF adaptou o código para módulo interno de sua crate
de impressão, preservou sua fachada e tratamento local das opções CUPS,
alterou a identidade dos trabalhos e o protocolo interno para NEXOPDF_PRINT_*,
e trouxe os testes de transporte para o workspace. Os arquivos alterados
possuem aviso de modificação. Licenças e NOTICE originais acompanham o código.

Esta incorporação elimina a busca Git durante a compilação. Ela é um
baseline derivado para a reconstrução incremental do transporte, não uma
declaração de autoria original de todo o subsistema.

Reconstrução do caminho Windows: `raster_job.rs` prepara folhas pelo renderizador
CPU do NEXOPDF; o script passa a receber imagens e dimensões físicas, sem executar
Windows.Data.Pdf. Isso evita o caminho de driver gráfico que apresentou falha
0xc0000005 em atidxx64.dll nesta máquina. A preparação usa até 600 dpi e 64 milhões
de pixels por folha, em blocos de 2048 pixels. O orçamento principal de imagem é
de até 256 MB, além das estruturas internas do renderizador. Trabalhos Windows
ficam limitados a 1.000 folhas e 1 GiB; erros sugerem dividir trabalhos maiores.
Parsers CUPS e helpers de configuração GDI ainda preservam sua origem declarada.
