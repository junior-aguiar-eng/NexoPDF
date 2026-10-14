# NEXOPDF — plano de implementação e correções

> Execução: usar superpowers:executing-plans, por etapa. Este documento organiza o programa de trabalho; decisões ainda não verificadas exigem investigação e um plano técnico específico antes da implementação correspondente.

**Objetivo:** eliminar a dependência de compilação de craft-libs, construir identidade visual própria, desenvolver os recursos pendentes e publicar uma versão validada por último.

**Arquitetura:** preservar engine, comandos, automação e formatos persistidos durante as mudanças de infraestrutura e interface. Desenvolver componentes locais e substituir os existentes por contratos verificados, sem reescrever funcionalidades independentes.

**Tecnologias verificadas:** Rust 1.95, edição 2024, Cargo workspace, egui/eframe 0.36, renderização wgpu com fallback glow no desktop; Windows, macOS, Linux, FreeBSD e WASM.

**Especificação de origem:** solicitação do proprietário nesta conversa em 2026-10-13; interface minimalista ultracontemporânea, visual marcante sem excesso de cores, glassmorphism leve e nenhuma perda de ferramentas. Mockups são propostas, não especificação visual aprovada para implementação.

## Restrições globais

- Publicação de release e instaladores somente na etapa final.
- Manter visibilidade pública e desenvolvimento fechado a contribuições externas.
- Não confundir independência de craft-libs com eliminação de bibliotecas Rust ou serviços locais do sistema operacional.
- Código incorporado/adaptado continua com licenças e atribuições pertinentes. Renomear não transforma autoria de terceiros em autoria original.
- Não prometer desenvolvimento clean-room: houve leitura prévia do código dessas dependências.
- Não remover comandos, ferramentas, atalhos, automação ou acessibilidade para simplificar a interface.
- Assets seguem AGENTS.md; manter proveniência e atribuição, sem imitar identidade visual de terceiros.
- Preservar documentos e configurações existentes; sem migrações destrutivas.
- Commits pequenos por mudança lógica, incluindo testes e documentação correspondentes. Sem reescrever o histórico.
- Commit, push e publicação são operações distintas; executar somente conforme autorização vigente.
- Atualizar evidências ao longo das etapas; nunca deixar documentação deliberadamente incorreta até a etapa 6.

## Riscos que os testes devem cobrir

1. Layout salvo inválido/antigo: recuperação previsível, sem perder acesso a ferramentas (etapa 2).
2. Impressora indisponível, nomes Unicode e falha de driver: erro acionável e limpeza de temporários (etapa 1).
3. GPU sem suporte ao efeito ou alto DPI: interface legível e operacional com efeitos desativados (etapa 2).
4. PDF complexo, criptografado ou malformado: erro seguro ou preservação do documento (etapas 3–5).
5. Instalação em ambiente limpo: assets, modelos e avisos legais disponíveis sem depender do checkout de desenvolvimento (etapa 7).

## 1. Independência de compilação e impressão própria

**Arquivos envolvidos:** Cargo.toml, Cargo.lock, deny.toml, crates/print/Cargo.toml, crates/print/src/spool.rs, crates/print/src/tests.rs, crates/print/README.md, xtask/src/layers.rs, NOTICE e avisos legais pertinentes.

**Contratos a preservar:** nexopdf_print::spool::{Printer, Job, Duplex, Scaling, printers, printer_options, submit, open_printer_preferences}; erros continuam expostos pelo PrintError do projeto.

- [x] Registrar baseline das revisões: craft-ui 0.1.0 em 5c874d51eeb3de70e735c8ce87a59a7dedd47675; craft-print 0.1.0 em d991bf6792f8ade99c51358bf3c1d902c8cd96b5.
- [ ] Capturar contratos e testes de comportamento antes de substituir código, incluindo permissões, composição e integração CLI/UI.
- [x] Tornar a UI atual local como ponte para a etapa 2, preservando origem e testes; usar nexopdf-ui em crates/ui e registrar sua camada L7 e a relação permitida com ui-egui. Não chamar essa ponte de implementação original.
- [ ] Investigar e especificar o transporte próprio de impressão a partir dos contratos e documentação pública do sistema, dentro da crate nexopdf-print existente. Não criar pacote homônimo nem substituir composição, seleção, livretos e pôsteres.
- [ ] Implementar por plataforma: Windows primeiro; depois CUPS para Unix; preservar a impressão web existente. Definir wrappers seguros antes de qualquer alteração de backend; não introduzir unsafe no workspace.
- [ ] Preservar escala física, orientação por folha, duplex, execução sem console, trabalho fora da thread da interface e limpeza de temporários em sucesso/falha.
- [x] Se o transporte continuar com PowerShell, manter Rust, script e testes sincronizados ao adotar NEXOPDF_PRINT_*; eliminar protocolo antigo somente depois da troca integral.
- [x] Preservar validação local de nomes de destinos e opções CUPS de fila/instância.
- [x] Retirar entradas Git de craft-libs do manifesto/lockfile e a exceção correspondente em deny.toml quando não houver consumidores.
- [x] Validar resolução local sem craft-libs e sem acesso à rede, usando dependências de registro previamente provisionadas em ambiente isolado; não apagar caches pessoais para testar.

**Conclusão:** nenhuma busca de craft-libs é necessária; impressão conserva os contratos testados nas plataformas suportadas. Impressão em hardware real é evidência separada dos testes simulados. Se reconstrução própria não alcançar equivalência, registrar a lacuna antes de encerrar a etapa.

## 2. Interface original NEXOPDF

**Áreas:** crates/ui (componentes locais), crates/ui-egui/src/{icons,chrome,docking}.rs e módulos de tema/preferências a localizar antes do subplano; apps/nexopdf para integração gráfica quando necessária; testes de UI/control e persistência; xtask/src/layers.rs.

- [ ] Inventariar todos os comandos, ferramentas, atalhos, menus, painéis e caminhos de automação atuais. Produzir matriz antes/depois com acesso equivalente.
- [ ] Refinar e aprovar proposta visual: janela principal, ferramenta expandida, diálogo, preferências de aparência e janela pequena/alto DPI. A imagem gerada não substitui essas especificações.
- [ ] Definir cores, tipografia, medidas, ícones Lucide, estados de foco/hover/seleção/desabilitado e critérios de contraste.
- [ ] Medir baseline gráfico e prototipar separadamente transparência, desfoque dentro da aplicação e eventual efeito nativo da janela. Não confundir os três.
- [ ] Definir opacidade dos painéis e intensidade do desfoque como controles separados; fundo sólido disponível. Texto, ícones e páginas permanecem nítidos e opacos.
- [ ] Implementar componentes próprios progressivamente: botões/ícones, abas, barras, painéis e docking. Preservar contratos e atribuições das partes ainda derivadas.
- [ ] Manter compatibilidade dos layouts persistidos; testar configuração inválida, restauração e acesso ao catálogo completo.
- [ ] Garantir alternativa sem desfoque para glow, WASM e hardware incompatível conforme os ensaios. Não impor serviço externo ou transparência do desktop como requisito.
- [ ] Validar visualmente a aplicação real, teclado, foco, leitores de tela quando disponíveis, DPI e matriz de comandos; comparar desempenho com o baseline.

**Conclusão:** identidade visual aprovada aplicada à UI, nenhuma ferramenta perdida e nenhuma mudança funcional involuntária. O grau de código original e de código preservado fica documentado honestamente.

## 3. Edição existente e conversão

**Áreas:** crates/edit, crates/content, crates/fonts, crates/create, crates/export, crates/engine, crates/automation, crates/ui-egui; docs/editing-existing-text.md e docs/file-format-parity.md.

- [ ] Reproduzir limitações atuais com fixtures sintéticas e conferir o que já existe em DOCX/XLSX/PPTX antes de planejar trabalho.
- [ ] Priorizar edição preservando fonte, estilo, posição e conteúdo não alterado; separar casos de texto rotacionado, fontes subsetadas e escrita contextual.
- [ ] Implementar correções com regressões de salvar/reabrir, undo/redo, comparação visual e automação.
- [ ] Melhorar layout das exportações existentes, validando em aplicativos consumidores reais quando disponíveis; distinguir teste estrutural de inspeção visual.
- [ ] Especificar Office para PDF: comparar solução local embutida e conversor externo opcional. Dependência nova relevante, tamanho, licença ou serviço exigem decisão antes de adoção.
- [ ] Integrar cada recurso pelo engine e CLI/MCP antes de marcar paridade como entregue.

**Conclusão:** cenários acordados passam de ponta a ponta; limitações restantes estão explícitas, sem promessa de fidelidade universal.

## 4. OCR com prioridade em português

**Áreas:** crates/ocr, crates/ml, integração engine/automation/UI, manifesto de modelos e empacotamento a identificar no subplano.

- [ ] Montar avaliação sintética de português: acentos, cedilha, números, colunas, tabelas, inclinação e baixa qualidade; registrar baseline e metas antes da escolha do motor/modelo.
- [ ] Avaliar motores/modelos locais, licenças, tamanho, memória e execução offline. Não pressupor que trocar modelo resolve todos os casos.
- [ ] Implementar português e pré-processamento com testes quantitativos e inspeção do texto/posicionamento.
- [ ] Tratar separadamente PDF pesquisável e texto efetivamente editável; preservar imagem original e possibilidade de desfazer.
- [ ] Avaliar OCR no navegador em subetapa própria, com limites de memória, cancelamento e alternativa explícita quando não suportado.

**Conclusão:** metas registradas de reconhecimento atingidas; modelos distribuídos legalmente e sem downloads inesperados durante o uso previsto.

## 5. Recursos avançados, em entregas separadas

**Áreas:** crates/sign, crates/a11y, crates/preflight, crates/optimize e respectivas interfaces/automação.

- [ ] Assinaturas: inventariar suporte real por plataforma, então definir certificados/tokens de teste, caminhos PKCS#11/CNG e tratamento de PIN. Nunca registrar segredos.
- [ ] Especificar consulta de revogação e carimbo de tempo: consentimento/configuração de rede, confiança, timeout e estados indisponível/desconhecido distintos de válido/inválido.
- [ ] Acessibilidade: evolução de tags, ordem de leitura e interface correspondente, com preservação do conteúdo e validação automática mais inspeção manual.
- [ ] Conformidade: escolher perfis e versões exatas de PDF/A, PDF/X e PDF/UA, implementar e testar cada perfil separadamente com validadores apropriados.
- [ ] Não converter ausência de falha em declaração de conformidade, assinatura confiável ou acessibilidade completa.

**Conclusão:** cada capacidade tem escopo, evidência e limites próprios. Esta fase exige subplanos; não é uma correção única.

## 6. Consolidação documental e auditoria final

**Arquivos:** ROADMAP.md, docs/roadmap.md, docs/gaps.md, docs/target-app-parity.md, parity/acrobat-features.toml, docs/development.md, docs/releasing.md e avisos legais.

- [ ] Consolidar atualizações feitas em todas as fases e remover contradições, duplicações e referências operacionais obsoletas.
- [ ] Registrar a validação FreeBSD já concluída e substituir alegações antigas por evidências atuais.
- [ ] Preservar procedência histórica necessária; não apagar nomes de terceiros de textos legais por branding.
- [ ] Separar implementado, testado automaticamente, validado visualmente e não comprovado.
- [ ] Revisar segurança, dependências, licenças, instalação e matriz completa de ferramentas.

**Conclusão:** documentação descreve o estado entregue; nenhum percentual histórico é apresentado como medição nova.

## 7. Publicação para download — última etapa

**Áreas:** workflows e scripts de release existentes, packaging, docs/releasing.md.

- [ ] Definir versão candidata, plataformas e artefatos realmente suportados; build reproduzível a partir do commit identificado, lockfile e entradas fixadas.
- [ ] Validar instalador e portátil Windows em ambiente limpo: abertura, instância única, fontes, OCR, impressão, atualização e desinstalação.
- [ ] Executar validação própria para cada outra plataforma anunciada; não extrapolar aprovação Windows para macOS/Linux/FreeBSD/web.
- [ ] Conferir inclusão de licenças, avisos, modelos e fonts, hashes dos artefatos e notas de versão.
- [ ] Verificar assinatura/notarização e credenciais disponíveis; registrar estado real, sem declarar pacote assinado quando não for.
- [ ] Após aceitação e autorização de publicação, criar release e conferir download/instalação dos artefatos publicados.

**Conclusão:** downloads acessíveis, artefatos identificados e instalação comprovada nas plataformas anunciadas.

## Verificação e unidades de entrega

Cada alteração funcional começa por reprodução/teste pertinente, recebe implementação mínima e termina com teste, revisão e documentação da mesma unidade lógica. Executar gates do projeto: fmt, Clippy, testes do workspace, layers, assets, parity, deny e WASM conforme os comandos existentes. Testes nativos e inspeção visual complementam esses gates.

Não repetir toda a matriz para cada edição documental nem iniciar publicação antecipadamente. Separar, por exemplo, independência da UI, impressão Windows, impressão CUPS, componentes visuais, aparência ajustável, cada correção de edição/conversão e cada capacidade avançada. Um commit não mistura essas frentes; a validação remota acompanha pushes autorizados.

**Estado deste plano:** implementação autorizada pelo proprietário. Etapa 1 em andamento
na branch `codex/nexopdf-evolucao`, a partir de `443b050`. Sem publicação.

### Evidências de execução (2026-10-13)

- UI local incorporada como ponte com proveniência, licenças e 73 testes unitários aprovados.
- Regra de isolamento da UI: falhou antes da migração e passou após o registro em L7.
- Cinco comparações visuais e o fluxo de docking passaram com DirectX 12 neste Windows.
- Resolução offline para Windows com Cargo home sem Git e registry pré-provisionado: zero pacotes Git.
- Transporte Windows em reconstrução: testes revelaram falha nativa em `atidxx64.dll`
  durante Windows.Data.Pdf. Preparação das folhas mudou para `nexopdf-render` CPU;
  44 testes de impressão aprovados, incluindo dimensões, rotação, UserUnit e pixels sintéticos. Validação local geral aprovada (resultados abaixo); hardware e plataformas Unix não comprovados neste ambiente. Não considerar equivalência de impressão em todas as plataformas demonstrada.
- Direção visual escolhida pelo proprietário: grafite com vidro discreto, sem brilho forte.

- Revisão independente final da impressão: nenhum defeito concreto adicional identificado; orçamento de imagem e limites por trabalho documentados.
- Clippy de todo o workspace (todos os alvos): aprovado. Suíte completa aprovada: 2.938 testes passaram e 14 foram ignorados.
- Assets e parity: aprovados; parity mantém aviso preexistente sobre 96 recursos sem ferramenta de automação.
- Inventário inicial da etapa 2: 23 grupos no catálogo e 188 comandos registrados no engine; os atalhos locais do canvas e painéis precisam de inventário complementar. Catálogo e registro continuam inalterados.

### Validação complementar (2026-10-14)

- Workspace nativo: 2.938 testes aprovados, 14 ignorados e nenhuma falha. Os ignorados dependem de rede, corpus/arquivos privados, certificados temporários ou medições específicas.
- Clippy do workspace com todos os alvos: aprovado; verificação final específica da impressão também aprovada, com 44 testes.
- Auditoria com cargo-deny 0.20.2 instalado apenas em target/evolucao/tools: advisories, bans, licenses e sources aprovados. O primeiro ensaio detectou a versão ausente da dependência local de UI; a versão 0.7.1 foi declarada, sem flexibilizar a política.
- Primeiro ensaio geral WASM falhou por reutilização parcial de cache entre diretórios (E0463 em num_enum). A ligação temporária foi removida. nexopdf-js passou tanto em um diretório novo quanto no diretório original do cache, sem alteração de código. Passagem completa WASM isolada aprovada: 32 crates. Há três avisos em código não alterado da interface; nenhum erro.
- Tela principal real renderizada e inspecionada em 1440×900, com DirectX 12 e fontes de sistema desativadas. Essa captura registra a UI anterior; o novo visual grafite ainda não foi aplicado.
