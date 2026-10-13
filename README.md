# NexoPDF

<p align="center">
  <img src="docs/brand/nexopdf-mark.svg" alt="NexoPDF Logo" width="96">
</p>

<p align="center">
  <b>Bancada Documental em PDF de Alta Performance · 100% Nativo em Rust</b><br>
  Leitura, organização, combinação, divisão, assinaturas e anotações em PDF de forma ultrarrápida, local e segura.<br>
  <b>Zero Telemetria · Privacidade Absoluta · Processamento Local</b>
</p>

<p align="center">
  <img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-d97706">
  <img alt="Written in Rust" src="https://img.shields.io/badge/written%20in-Rust-b45309">
  <img alt="Platforms: Windows · macOS · Linux" src="https://img.shields.io/badge/platforms-Windows%20%C2%B7%20macOS%20%C2%B7%20Linux-f59e0b">
  <img alt="No account, no telemetry" src="https://img.shields.io/badge/privacidade-100%25%20local-d97706">
</p>

---

## ✦ Visão Geral

O **NexoPDF** é uma bancada de trabalho completa e offline para visualização, manipulação e gestão de documentos PDF. Desenvolvido inteiramente em **Rust**, o NexoPDF combina fidelidade tipográfica estrita com máxima velocidade e proteção de dados: nenhum dado ou telemetria sai do seu computador.

### Destaques do NexoPDF

<table>
<tr>
<td width="33%" valign="top">

### ⚡ Fidelidade e Rapidez
Renderização de alta precisão com suporte a scripts mundiais, kerning, máscaras e transparências, inicialização instantânea e consumo eficiente de memória.

</td>
<td width="33%" valign="top">

### 🔒 Segurança e Imutabilidade
Suas alterações são salvas de forma atômica e não destrutiva. O arquivo original é preservado e histórico de desfazer profundo protege contra perda de dados.

</td>
<td width="33%" valign="top">

### 🛡️ Privacidade Absoluta
Sem contas, sem nuvem, sem conexões externas ocultas. Opera 100% offline direto na sua máquina, garantindo total conformidade e sigilo.

</td>
</tr>
</table>

---

## 🛠️ Recursos Principais

- **Visualização Completa**: Modos contínuo, página única, duas páginas lado a lado (leitura), rotação de visualização e tela cheia.
- **Organização Visual de Páginas**: Reordenação intuitiva, rotação, extração e exclusão de páginas.
- **Combinação e Divisão**: Mesclagem de múltiplos PDFs e divisão cirúrgica por intervalos de páginas sem perdas de metadados.
- **Anotações e Marcações**: Realces de texto, notas, marcações à mão livre, formas geométricas e caixas de texto.
- **Formulários Interativos**: Preenchimento completo de formulários PDF (AcroForms), com suporte a validação e exportação/importação XFDF.
- **Assinaturas Digitais**: Suporte a assinaturas de documentos com certificados digitais e validação criptográfica.
- **Otimização e Auditoria**: Redução inteligente de tamanho de arquivos, compressão de imagens e remoção de elementos órfãos.
- **Tema Âmbar Sofisticado**: Interface moderna em tons âmbar e grafite, suave para leitura prolongada.

---

## 🚀 Como Compilar e Executar

### Pré-requisitos
- **Rust 1.95+** (recomenda-se canal estável atualizado)
- **Git**
- Sistema Operacional: Windows 10/11 x64, macOS ou Linux

### Compilação do Aplicativo Desktop

```bash
# Clonar o repositório
git clone https://github.com/junior-aguiar-eng/NexoPDF.git
cd NexoPDF

# Verificar dependências e compilar
cargo check -p nexopdf
cargo build -p nexopdf

# Executar o NexoPDF
cargo run -p nexopdf
```

No Windows, você também pode simplesmente dar duplo clique em `iniciar-nexopdf.bat`.

---

## 📂 Estrutura do Workspace

O projeto é organizado como um workspace modular em Rust:

- **`apps/nexopdf`**: Aplicativo desktop nativo com interface gráfica (egui/wgpu).
- **`apps/nexopdf-cli`**: Utilitário de linha de comando para automação em lote.
- **`crates/ui-egui`**: Componentes de interface gráfica, docked panels e temas visuais.
- **`crates/engine`**: Núcleo de execução e orquestração de comandos documentais.
- **`crates/render`**: Engine de rasterização e desenho vetorial de PDFs.
- **`crates/model`**, **`crates/cos`**, **`crates/crypt`**: Manipulação do formato PDF, criptografia e serialização de objetos.
- **`crates/organize`**, **`crates/forms`**, **`crates/annot`**, **`crates/sign`**: Funcionalidades especializadas de documentos.

---

## 📄 Licença

O NexoPDF é distribuído sob licença dupla **MIT** ou **Apache-2.0**. Consulte os arquivos [LICENSE-MIT](LICENSE-MIT) e [LICENSE-APACHE](LICENSE-APACHE) para mais informações.
