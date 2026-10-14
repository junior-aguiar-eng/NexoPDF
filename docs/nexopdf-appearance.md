# Aparência NEXOPDF: grafite e superfícies discretas

Direção escolhida pelo proprietário em 2026-10-14: grafite, vidro discreto e sem brilho forte.

Esta primeira entrega visual aplica uma paleta fria neutra, contraste de texto preservado,
ícones monocromáticos de ferramentas e uma composição própria da tela inicial. O modo claro
continua disponível e as preferências existentes continuam válidas. O catálogo do engine,
comandos, atalhos, menus e painéis não são substituídos.

O acabamento da tela inicial usa superfícies translúcidas sobre um fundo interno estático.
A opacidade admite 70–100%, padrão 90%; 100% produz superfícies sólidas. Texto, ícones e
páginas não recebem transparência. A configuração é persistida e exposta pelo controle
`ui.set` (`glass_opacity`), sem ativar o canal de controle automaticamente.

**Limite desta entrega:** transparência de superfícies não é desfoque. Não há captura do
desktop, blur de documentos nem novo shader de backdrop blur. O desfoque real e a revisão
dos demais componentes originais continuam em etapas posteriores. Os componentes derivados
em `crates/ui` conservam seus avisos e proveniência.

A validação deve cobrir configurações inválidas, persistência, controle headless, catálogo
completo e capturas claras/escuras em resoluções distintas. As capturas locais não demonstram
instalação nem uso de impressora física.

Validação dirigida inicial: 429 testes aprovados e um ignorado. Capturas reais em
1440×900 (grafite e claro sólido), 800×720 (opacidade 70%) e preferências a 150%
foram inspecionadas com DirectX 12 e fontes de sistema desativadas. A captura estreita
revelou sobreposição preexistente da busca com os modos, tratada em correção separada.

O catálogo permanece com 23 grupos e 188 comandos; isso comprova preservação do registro,
não validação funcional de todas as operações em documentos reais. Blur, revisão de todos
os componentes derivados e medição comparativa de desempenho continuam pendentes.

| Área preservada | Base 443b050 | Nova aparência | Evidência |
| --- | ---: | ---: | --- |
| Grupos de ferramentas | 23 | 23 | Catálogo idêntico; teste de acesso após Ver mais |
| Comandos registrados | 188 | 188 | Registro do engine idêntico |
| Modos de trabalho | 5 | 5 | Enumeração e ações preservadas |
| Painéis laterais direitos | 11 | 11 | Enumeração preservada |
| Famílias de ferramentas rápidas | 18 | 18 | Enumeração preservada, incluindo variantes com subferramentas |
| Variantes de diálogo | 43 | 43 | Enumeração preservada |
| Atalhos e interação do canvas | preservados | preservados | Arquivos de comandos/canvas idênticos; testes de foco e teclado |

A comparação estática não prova equivalência funcional de cada variante. A bateria de
interação complementa o inventário; validação assistiva e de todas as plataformas permanece
separada.

## Validação final local — 2026-10-14

- Formatação, Clippy do workspace com todos os alvos, WASM da interface, camadas, assets e parity aprovados. O WASM mantém três avisos preexistentes; parity mantém 96 recursos sem ferramenta de automação.
- 2.941 testes funcionais aprovados, 14 ignorados, com dois benchmarks explicitamente filtrados; os dois benchmarks passaram depois com o mesmo binário e sem alterar código ou limites: comentários 36,99 ms, páginas 13,28 ms, campos 3,90 ms e rolagem 28,83 ms, abaixo de 40 ms.
- As rodadas anteriores de desempenho falharam (comentários 52–55 ms, rolagem 42–44 ms); houve carga concorrente observada. O resultado final não elimina a variabilidade nem substitui comparação controlada com a base.
- Uma falha de linkagem LNK1120 em bookmarks desapareceu após cargo clean limitado a nexopdf-ui-egui, sem contorno no código.
- A CI da alteração visual ainda precisa executar; a aprovação do run 38508246562 pertence à base de componentes locais.
