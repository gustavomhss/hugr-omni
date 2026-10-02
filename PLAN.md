# hugr-omni — Plano de execução

> Status: **G0 assinado (B: núcleo próprio, TS primeiro)** · contrato da API aguardando aprovação do Owner (D5) · CI bloqueado por billing da org · 2026-10-01
> Repo: `HuGR-Labs/hugr-omni` (público) · Licença: MIT OR Apache-2.0
> Lead/orquestrador: Claude (sessão principal). Execução: sub-agentes Claude. Revisão: Codex CLI.

---

## 0. Resumo em 30 segundos

- **Produto:** uma biblioteca que roda processos e terminais com o mesmo comportamento em Windows, macOS e Linux. O núcleo é em Rust, com pacotes idiomáticos para TypeScript (a interface principal; Node, Bun e Deno), Python e Rust desde o dia 1.
- **Como provamos que funciona:** duas camadas.
  - Um **contrato pequeno** (~35 testes determinísticos, um por promessa do GUARANTEES), escrito uma vez e executado nas 3 linguagens.
  - **QA de uso real medido por KPIs**: agentes rodando suítes de teste de verdade, dev servers e terminais interativos, comparados com o stdlib de cada linguagem. Os KPIs (seção 4) decidem o release.
- **Como construímos rápido:** 31 work packages (WPs), até 6 agentes em paralelo, cada um em worktree própria, com o contrato da API congelado antes. Os WPs se juntam em **7 PRs em bundle**, e o CI completo roda uma vez por bundle.
- **Como garantimos qualidade:** cada WP é revisado a frio pelo Codex **contra o próprio card** (completude, sucesso, invariantes, qualidade, DoD, com veredito campo a campo) e verificado pelo lead antes de entrar no bundle.
- **Como evitamos overengineering:** uma API com no máximo 16 conceitos, a regra de paridade (cada feature custa 3×) e um "não fazer" explícito em cada WP. Nada de teste por teste.
- **Como o código fica organizado:** monolito modular, com módulos de responsabilidade única e dependências apontando só para baixo. Nenhum arquivo de código passa de 650 linhas: o ideal é ≤400, até 600 está ok, 601–650 é tolerado com aviso. O `scripts/file-size-guard.py` falha o gate acima disso (documentos ficam fora).
- **Prazo (estimativa grosseira):** Fase 0 em 3–5 dias; Fase 1 em ~2 semanas de relógio. O release depende dos design partners.

---

## 1. Decisões do Owner

| # | Decisão | Estado |
|---|---|---|
| D1 | Nome | ✅ `hugr-omni` (livre em npm, PyPI e crates.io) |
| D2 | Repo | ✅ `HuGR-Labs/hugr-omni`, público |
| D3 | Pasta local | ✅ `~/Documents/HuGR/hugr-omni` |
| D4 | Licença | ✅ MIT OR Apache-2.0 |
| D5 | **Aprovar o contrato da API (seção 3)** | ⏳ pendente: bloqueia o W00, não bloqueia a Fase 0 |
| D6 | Estratégia de testes: contrato enxuto + QA com KPIs | ✅ diretriz do Owner (2026-10-01) |
| D7 | PRs em bundle para economizar CI | ✅ diretriz do Owner (2026-10-01) |
| D9 | G0: núcleo próprio enxuto, TS primeiro (opção B; `docs/decisions/G0.md`) | ✅ assinado pelo Owner (2026-10-01) |
| D10 | Linguagens: TypeScript (Node/Bun/Deno) no v0.1; Python e Rust (pacotes publicados) no v0.2 | ✅ Owner (2026-10-01) |
| D11 | Destravar o billing do GitHub Actions da HuGR-Labs | ⏳ pendente: sem isso não há prova em Windows |
| D8 | Monolito modular + god-file guard (400 ideal · 600 ok · 650 máximo por arquivo de código; não vale para documentos; é por arquivo, não por PR) | ✅ diretriz do Owner (2026-10-01) |

Também ficam com você, em paralelo e sem bloquear o build:
- conversas com ≥5 maintainers (G0-05);
- configurar trusted publishing nos registries (o passo a passo vem do S3);
- assinar o gate G0 e a publicação.

---

## 2. Princípios

**O usuário é o rei.** Quem usa é um dev construindo um agente ou uma ferramenta.

- **UX-A** O caso de 80% funciona sem nenhuma opção.
- **UX-B** Existe um jeito óbvio de fazer cada coisa.
- **UX-C** Os nomes são idiomáticos em cada linguagem.
- **UX-D** Todo erro diz o que falhou, com qual valor, e como consertar.
- **UX-E** Nenhuma surpresa entre sistemas: ou o comportamento é igual, ou a diferença está escrita na doc da função.
- **UX-F** Instala sem compilar, em menos de 30 s.
- **UX-G** O hover na IDE mostra a doc e um exemplo.

**Cuidado com overengineering e com excesso de teste.**

- Sem abstração antes do segundo uso real.
- Sem opção fora do contrato.
- Spikes produzem decisões, não código para main.
- Testes determinísticos só onde guardam uma promessa pública. O resto se prova usando de verdade e medindo.

**Monolito modular, sem god files.** Um núcleo único (uma crate), dividido em módulos de responsabilidade única com dependências só para baixo (seção 3, "Arquitetura interna"). Arquivo de código: ideal ≤400 linhas, ok até 600, máximo absoluto 650. Passou disso, o arquivo é dividido; não existe lista de exceções.

**Rigor não se negocia.** Gate vermelho se corrige na raiz. Nenhuma dívida ou supressão entra sem um waiver escrito e assinado pelo Owner.

---

## 3. Contrato da API (proposta: aprovar = D5)

### TypeScript (interface principal: Node, Bun, Deno)

```ts
import { run, spawn } from "hugr-omni";

// 1. Rodar e coletar: o caso mais comum
const r = await run("npm", ["test"], { cwd: repo, timeoutMs: 120_000 });
if (!r.success) console.log(r.reason, r.exitCode, r.stderr);

// 2. Streaming + cleanup garantido ao sair do escopo
{
  await using dev = spawn("npm", ["run", "dev"]);
  for await (const { stream, data } of dev.output) {
    if (data.includes("ready")) break;   // parar de ler nunca trava o processo
  }
} // aqui npm, node e tudo que eles abriram já morreram

// 3. Terminal interativo
const sh = spawn("bash", [], { pty: { cols: 120, rows: 30 } });
await sh.write("ls\n");
sh.resize(100, 40);
const exit = await sh.kill();             // pede com educação, força após graceMs
```

### Python (sync e asyncio)

```python
from hugr_omni import run, spawn, aio

r = run(["npm", "test"], cwd=repo, timeout=120)
if not r.success:
    print(r.reason, r.exit_code, r.stderr)

with spawn(["npm", "run", "dev"]) as dev:
    for chunk in dev.output:
        if "ready" in chunk.data:
            break
# saiu do with: árvore inteira morta

r = await aio.run(["pytest"], timeout=300)
```

### Rust (tokio)

```rust
use hugr_omni::Command;
let r = Command::new("npm").arg("test").cwd(repo)
    .timeout(Duration::from_secs(120)).run().await?;
let mut dev = Command::new("npm").args(["run", "dev"]).spawn()?;
while let Some(chunk) = dev.output_text().next().await { /* ... */ }
drop(dev); // mata a árvore
```

### Glossário (16 conceitos; mudar exige aprovação do Owner; paridade verificada por máquina)

| Conceito | TS | Python | Rust |
|---|---|---|---|
| rodar e coletar | `run(cmd, args, opts)` | `run([cmd, *args], **kw)` / `aio.run` | `Command::run()` |
| rodar com streaming | `spawn(...)` → `Child` | `spawn(...)` / `aio.spawn` | `Command::spawn()` |
| diretório | `cwd` | `cwd` | `.cwd()` |
| ambiente (merge sobre o herdado; `null` remove) | `env` | `env` | `.env()` / `.env_remove()` |
| ambiente limpo | `inheritEnv: false` | `inherit_env=False` | `.env_clear()` |
| terminal | `pty: true \| {cols, rows}` | `pty=True \| (cols, rows)` | `.pty(PtySize)` |
| timeout | `timeoutMs` | `timeout` (s) | `.timeout(Duration)` |
| graça do kill | `graceMs` (2000) | `grace` (2.0) | `.grace(Duration)` |
| cancelamento | `signal: AbortSignal` | cancelar a task / `KeyboardInterrupt` | `.cancel_on(CancellationToken)` |
| stdin do `run` | `input` | `input` | `.input()` |
| limite de saída do `run` | `maxOutputBytes` (16 MiB/stream) | `max_output_bytes` | `.max_output_bytes()` |
| texto vs bytes | `encoding: "utf8" \| "bytes"` | `text=True \| False` | `output_text()` / `output()` |
| Child | `pid · output · write · end · resize · kill · wait · droppedBytes` | idem, em snake_case | idem |
| Exit | `exitCode · signal · reason · success` | `exit_code · signal · reason · success` | `Exit` |
| RunResult | Exit + `stdout · stderr · truncated` | idem | `RunOutput` |
| erro | `OmniError` com `.code` | `OmniError` + subclasses (`CommandNotFoundError` também é `FileNotFoundError`) | `Error` `#[non_exhaustive]` |

**Códigos de erro:** `NOT_FOUND · NOT_EXECUTABLE · INVALID_CWD · INVALID_ARGUMENT · ABORTED · CLOSED · IO`. A Fase 2 acrescenta `SANDBOX_UNAVAILABLE`.

**`reason`:**
- `exit`: o processo terminou sozinho;
- `signal`: foi morto por fora (Unix);
- `killed`, `timeout`, `aborted`: fomos nós que matamos.

Numa corrida entre eventos, ganha o que aconteceu primeiro. No Windows, códigos de saída acima de 255 (por exemplo `0xC000013A`) aparecem como inteiro não negativo em todas as linguagens.

**Defaults seguros:**
- `run()` entrega stdin fechado: um programa que espera input recebe EOF em vez de travar o agente.
- `env` faz merge sobre o ambiente herdado.
- A busca do programa usa o PATH final do filho.
- `kill()` sempre mata a árvore: gracioso primeiro, força depois de `graceMs`.
- Sair do escopo mata a árvore.
- O GC nunca mata um processo vivo. Quando o host sai, ele mata o que sobrou, netos órfãos incluídos (os níveis de garantia estão no GUARANTEES).
- Em TS, um `Child` vivo mantém o event loop vivo, como o `child_process`.
- Saída não lida fica num buffer de até 1 MiB por stream; o excedente é descartado e contado em `droppedBytes`. O filho **nunca** trava porque ninguém está lendo.
- Se a raiz sai mas um descendente segura o pipe, `run()` devolve o que tiver lido em até `graceMs` e mata o resto. Para processos de fundo de longa duração, use `spawn()`.
- Nunca há shell.

### Arquitetura interna (monolito modular)

Uma crate (`hugr-omni`), um módulo por responsabilidade, cada módulo com um `mod.rs` que é a sua fachada (o seam congelado no W00) e arquivos pequenos atrás dela:

| Módulo | Responsabilidade | Pode depender de | Dono |
|---|---|---|---|
| `api/` | superfície pública: `Command`, `Child`, `run`, tipos (`Exit`, `Chunk`, ...) | `process`, `io`, `spawn`, `pty`, `error` | W00 (congelado) |
| `process/` | ciclo de vida: árvore e kill (`tree`), `Exit` (`exit`), timeout/cancelamento (`supervise`), limpeza na saída do host (`registry`) | `spawn`, `io`, `pty`, `sys`, `error` | W07, W09 |
| `spawn/` | do pedido ao comando pronto: `resolve`, `env`, `validate` | `sys`, `error` | W03 |
| `io/` | saída (`out`, `decode`), stdin (`stdin`), coleta do `run` (`collect`) | `error` | W10 |
| `pty/` | sessão PTY (`session`) + backends `unix/` e `windows/` | `sys`, `io`, `error` | W12, W12w |
| `sys/` | primitivas do OS: `unix/` (grupo, sinais, wait) e `windows/` (Job, linha de comando, spawn) | `error` | W05, W06 |
| `error/` | `Error` + códigos (`mod.rs`, congelado) e textos (`messages`) | — | W00, W03 |
| `sandbox/` (Fase 2) | `policy` + backends `macos`, `linux/`, `windows/` | `sys`, `error` | SB1–SB6 |

Regras:
- As dependências só apontam para baixo na tabela. `sys/` e os backends de `pty/` nunca importam `api`, `process` ou `spawn`.
- `lib.rs` só reexporta `api` e `error`; o resto é `pub(crate)`.
- Os bindings são crates separadas e só enxergam a API pública (o compilador garante). Dentro de cada binding a divisão é por responsabilidade: `child`, `run`, `error`, `convert`.
- Um arquivo que encosta em 600 linhas é dividido dentro do mesmo módulo antes de virar problema.

---

## 4. Como provamos que funciona

### 4.1 Camada 1: contrato (determinístico, enxuto)

- São **~35 itens** (Apêndice A), um por promessa pública.
- Escritos **uma vez** como cenários JSON e executados pelos runners de Rust, TS (Node, Bun, Deno) e Python.
- Servem de especificação executável e de base do GUARANTEES.
- **Não** escrevemos teste unitário por exigência. O agente escreve se isso acelera o próprio trabalho, sem contar como critério.
- Teste flaky é corrigido ou apagado, nunca re-executado até passar.

### 4.2 Camada 2: QA de uso real + KPIs (decide o release)

Workloads reais, executados por um agente "usuário" scriptado em cada linguagem e comparados com o stdlib da mesma linguagem (`child_process`, `subprocess`, `std::process`):

| Workload | O que faz |
|---|---|
| QA-A Suítes reais | roda a suíte de teste de 3 projetos open source fixados por commit (um por ecossistema, < 60 s, sem rede) com timeout |
| QA-B Dev servers | Vite (Node) e Flask com reloader (Python, que cria processo filho): espera "ready", para, confere a árvore |
| QA-C Interativo | bash/zsh/pwsh, REPL do Python e do Node via PTY: comandos, Ctrl-C num comando longo, `exit` |
| QA-D Mal-comportados | processo que vira daemon, ignora SIGTERM, inunda saída, espera stdin (`git commit` abrindo editor), pede input |
| QA-E Loop de agente | 200 comandos típicos de agente (git, grep, build, test, install) com cancelamentos aleatórios |

| KPI | Meta para release |
|---|---|
| **K1** processos órfãos após cada tarefa | **0** |
| **K2** travamentos (tarefa sem retorno após timeout + graça) | **0** |
| **K3** latência de stop (kill → árvore morta), p95 | ≤ graceMs + 500 ms |
| **K4** overhead de spawn (latência até o PID e ida-e-volta de um filho trivial), p50 | ≤ 1,25× o stdlib |
| **K5** bytes de saída perdidos ou corrompidos (dentro dos limites documentados) | **0** |
| **K6** soak de 1000 tarefas: fds/handles voltam ao baseline; crescimento de RSS | ≤ 10 MB |
| **K7** crashes do host | **0** |
| **K8** paridade: K1–K7 no Windows | mesmas metas que macOS/Linux |
| **K9** instalação limpa: 5 alvos × npm/bun/deno/pip/uv/cargo | 100% |
| **K10** usuário frio: tarefas concluídas só com o README; mediana até o primeiro sucesso | 15/15; ≤ 5 min |
| **K11** design partners | ≥2 linguagens; código de plataforma removido; bugs de Windows com repro antes/depois |

O baseline do stdlib roda os mesmos workloads e registra os K1/K2 dele. A diferença é a prova de valor do produto, e vira o material do README.

### 4.3 Invariantes globais (INV)

- **INV-01** Semântica de processo só no núcleo Rust. Os bindings convertem tipos e modelo async, nada mais.
- **INV-02** Paridade: uma capacidade (função, opção ou campo) existe nas 3 linguagens ou em nenhuma.
- **INV-03** Nunca shell. `.cmd`/`.bat` no Windows: escaping seguro ou recusa.
- **INV-04** `kill` = árvore.
- **INV-05** Nada silencioso: erro tipado, código e mensagem acionável; perda de dados sempre contada.
- **INV-06** Nenhum panic atravessa FFI. Nada de `unwrap`/`expect` em código de biblioteca sem uma invariante provada.
- **INV-07** Nunca bloquear o host (thread principal do Node, GIL, executor tokio).
- **INV-08** Sem vazamento de processo, fd ou handle, inclusive em caminhos de erro.
- **INV-09** Garantia honesta: cada linha do GUARANTEES tem teste ou KPI medido naquele OS.
- **INV-10** Os defaults são a escolha segura (seção 3).
- **INV-11** `unsafe` só em `sys/`, `pty/`, `sandbox/` e na fronteira FFI, sempre com `// SAFETY:`.
- **INV-12** Dependência nova só com aprovação do lead (allowlist no `AGENTS.md`).
- **INV-13** Testes de contrato são somente-leitura para quem implementa.
- **INV-14** Nenhum arquivo de código rastreado passa de 650 linhas (`scripts/file-size-guard.py`; documentos e dados ficam fora). Não há lista de exceções.
- **INV-16** A saída de um processo filho nunca é observada pelo reaper de SIGCHLD do tokio: usa `waitpid` bloqueante por filho ou pidfd (Linux ≥ 5.3). Motivo: o S3 (Q8) mediu travamento determinístico dentro de Node e Bun no Linux sem pidfd.
- **INV-15** Camadas do monolito: dependências só para baixo na tabela "Arquitetura interna"; nenhum módulo importa uma camada acima.

### 4.4 Quality standards globais (QS)

- **QS-01** Rust: `rustfmt`, `clippy -D warnings`, `#![deny(missing_docs)]`. Nenhum `#[allow]` sem justificativa aprovada.
- **QS-02** TS: `strict`, zero `any`, JSDoc com exemplo em `run`/`spawn`.
- **QS-03** Python: `.pyi` completos, `pyright --strict`, `ruff`.
- **QS-04** Testes sincronizam por marcadores do fixture, nunca por `sleep`.
- **QS-05** Mensagens de erro: o quê + valor + causa provável + correção.
- **QS-06** O diff toca só o write-set do WP. Commits convencionais.
- **QS-07** Código simples: sem abstração sem segundo uso, sem opção fora do contrato, sem teste sem valor.
- **QS-08** Tamanho de arquivo: ideal ≤400 linhas, ok até 600; entre 601 e 650 o guard avisa e a revisão pede o plano de divisão.

### 4.5 Definition of Done global (por WP)

- **D1** Itens de contrato do WP verdes localmente: macOS nativo + Linux via Docker; código Windows compila com `clippy --target x86_64-pc-windows-msvc`.
- **D2** Nenhum item de contrato que já estava verde regrediu.
- **D3** Gates reproduzidos a frio pelo lead.
- **D4** WPs do núcleo e dos bindings: QA rápido local (QA-A + QA-D) sem piora de K1, K2 e K5.
- **D5** Codex: veredito `pass` nos 5 campos do card e zero P0/P1.
- **D6** O lead faz mutation probe em 1 item do WP (quebra, vê vermelho, restaura).
- **D7** Só o write-set mudou (`git diff --name-only`).
- **D8** Doc e CHANGELOG atualizados se o comportamento público mudou.
- **D10** `python3 scripts/file-size-guard.py` verde (nenhum arquivo acima de 650) e nenhuma importação que viole as camadas (INV-15).
- **D9** O agente para em "branch `wp/<id>` empurrada, verde localmente, aguardando o lead". Quem integra no bundle é o lead.

Windows em runtime e os alvos arm64 são provados no CI do bundle. Uma falha lá volta para o WP dono do item.

---

## 5. Orquestração

### Papéis

| Quem | Faz | Não faz |
|---|---|---|
| **Lead** (sessão principal) | decide, congela o contrato, escreve briefs, verifica, integra bundles, roda o QA, mantém este plano | não implementa WP de produto; não aceita relato sem verificar |
| **Agentes Claude** | executam 1 WP cada, em worktree isolada | não decidem interface, não saem do write-set, não abrem PR, não mergeiam |
| **Codex** | revisa cada WP contra o card; revisa cada bundle nos seams; red team da sandbox | não escreve código de produto |
| **Owner** | D5, conversas, G0, publicação | — |

**Modelos:**
- **Opus:** WPs com OS nativo, concorrência ou FFI (S1, S2, W01, W03, W05, W06, W07, W09, W10, W12, W12w, W13, W15, Q1, S4, SB2–SB4, SB6).
- **Sonnet:** pesquisa, runners, empacotamento, docs (R1, R2, S3, W02, W14, W18, Q2, SB1).

### Ciclo de cada WP

1. **Pré-dispatch (V1 do lead):** zero decisão em aberto, write-set disjunto, contrato congelado, gates escritos, baseline fixado.
2. **Dispatch:** `Agent` com `isolation: "worktree"`, branch `wp/<id>`, brief no formato do Apêndice C. O agente dá push no primeiro commit que compila (é ponto de recuperação, e não dispara CI).
3. **Agente:** implementa, roda os gates locais, empurra e **para**.
4. **Lead L0:** HEAD descende do baseline, commit existe, lista de arquivos ⊆ write-set.
5. **Codex, contra o card** (Apêndice D): veredito nos 5 campos + achados. P0/P1 voltam ao mesmo agente (SendMessage) e o Codex revisa de novo.
6. **Landing no bundle (lead):** gates a frio, QA rápido quando aplicável, mutation probe, squash de `wp/<id>` em `bundle/B<n>` (1 commit por WP).
7. Atualizar a tabela de status (seção 8).

**Concorrência:** até 6 agentes ao mesmo tempo, e no máximo 3 WPs prontos esperando revisão.

### Economia de CI e PRs em bundle

- Branches `wp/*` e `bundle/*` **não disparam CI**. Os workflows rodam só em: `pull_request` não-draft para `main`, `push` em `main` e `workflow_dispatch`.
- Cada bundle é um PR `bundle/B<n> → main`, aberto em **draft**:
  - CI rápido quando marcado como pronto (3 OS × contrato Rust, ~10 min);
  - CI completo uma única vez, com o label `full`: 5 alvos × runtimes + instalação limpa + QA no Windows.
- `concurrency: cancel-in-progress`; filtros de caminho (mudança só de docs roda só o check de docs); `Swatinem/rust-cache`.
- **Local primeiro:** macOS nativo (esta máquina é x86_64), Linux via Docker, Windows compilado por `clippy --target`.
- **Windows sob demanda:** `gh workflow run windows.yml -f filter=<testes>` (um job só), para WPs com muito Windows (W06, W12w).
- O merge do bundle usa rebase-merge, para cada WP continuar um commit próprio em `main`.
- O Codex revisa cada bundle uma vez, olhando só os seams entre WPs.

| Bundle | Conteúdo | CI |
|---|---|---|
| B0 | Fase 0: pesquisa + ADRs (só docs) | check de docs; cada spike tem um workflow próprio, disparado só por push no seu branch `spike/*` (o código do spike nunca entra em main) |
| B1 | Fundação: W00, W01, W02 | rápido |
| B2 | Núcleo: W03, W05, W06, W07, W09, W10 | rápido → completo |
| B3 | PTY: W12, W12w | rápido → completo |
| B4 | Linguagens + pacotes: W13, W15, W14 | completo |
| B5 | Docs + QA: W18, Q1, Q2 e correções vindas do QA | completo + QA Windows |
| B6 | Release: W21 | release |
| B7–B8 | Sandbox macOS/Linux (S4, SB1–SB4); Windows (SB6) | completo |

---

## 6. Ondas (DAG; a ordem é de integração, não uma barreira de dispatch)

```text
Fase 0   R1 · R2 · S1 · S2 · S3   (+ H0)          → WG0 gate (Owner assina)   [B0]
Fase 1   W00 (lead) → W01 · W02                                               [B1]
         → W03 · W05 · W06 · W10 → W07 → W09                                  [B2]
         → W12 · W12w                                                         [B3]
         → W13 · W14   (W15 Python e publicação Rust no v0.2)                 [B4]
         → W18 · Q1 → Q2 → correções                                          [B5]
         → W21                                                                [B6]
Fase 2-3 S4 → SB1 → SB2 · SB3 → SB4 → SB6                                     [B7, B8]
```

---

## 7. Work packages

> Cada card tem **Completude** (itens que precisam ficar verdes), **Sucesso** (o que o usuário observa), **Invariantes**, **Qualidade** e **DoD**. Só aparece o que é específico do WP; as seções 4.3–4.5 valem sempre. O `scripts/plan-sections-check.py` falha se algum card não tiver um desses campos (PLAN-01).

### Fase 0: validação

#### H0 · Owner: decisões, conversas, aprovações
- **Quem:** você · **Depende:** — · **Escreve:** `docs/research/conversations.md`, `docs/partners/**`
- **Completude:** G0-05, UX-02 (D5), REL-02.
- **Sucesso:** ≥5 conversas com dor, workaround, disposição de trocar e bloqueios; ≥2 partners em linguagens diferentes.
- **Invariantes:** nenhum dado pessoal além do nome do projeto.
- **Qualidade:** cada partner com medida antes/depois (código de plataforma removido, repros de bugs de Windows).
- **DoD:** registros commitados no B0 (conversas) e no B5 (partners).
- **Não fazer:** bloquear o build esperando as conversas.

#### R1 · Mapa de alternativas — *em execução*
- **Agente:** Sonnet · **Revisor:** Codex · **Depende:** — · **Escreve:** `docs/research/landscape.md`
- **Completude:** G0-01.
- **Sucesso:** tabela candidato × 22 comportamentos com links; a melhor combinação por linguagem; veredito: alguém chega a ≥80%?
- **Invariantes:** toda célula "sim" tem link; nada é instalado ou executado.
- **Qualidade:** só fontes primárias; status de manutenção de cada candidato.
- **DoD:** o Codex confere 30% das citações e o lead confere 10 sorteadas.
- **Não fazer:** propor arquitetura.

#### R2 · Evidência de dor — *em execução*
- **Agente:** Sonnet · **Revisor:** Codex · **Depende:** — · **Escreve:** `docs/research/pain.md`
- **Completude:** G0-02.
- **Sucesso:** contagem por repo × categoria em 8 agentes open source (TS, Python, Rust); top-10; fatia de issues de Windows; veredito por ecossistema.
- **Invariantes:** toda contagem é reproduzível pela query registrada; só leitura (nunca comentar).
- **Qualidade:** dedupe; classificação pelo corpo da issue.
- **DoD:** o lead reproduz 3 contagens e confere 5 classificações.
- **Não fazer:** contatar autores.

#### S1 · Spike: modelo de processo e ciclo de vida do host
- **Agente:** Opus · **Revisor:** Codex · **Depende:** repo · **Escreve:** `spikes/process/**` (branch, nunca vai para main), `docs/adr/0001-process-model.md`, `docs/adr/0002-host-exit.md`
- **Completude:** G0-03a. Responde com CI nos 3 OS:
  1. Job no Windows sem corrida (e se `std::process` basta, ou se é preciso `CreateProcessW` + quoting próprio de `.bat`).
  2. Jobs aninhados.
  3. Tier do encerramento gracioso no Windows.
  4. `setsid` vs `setpgid` e o limite de fuga.
  5. Cleanup quando o host morre, por OS.
  6. Ctrl-C/SIGTERM no host sem quebrar o tratamento de sinais do Node e do Python.
  7. Reuso de PID.
  8. Baseline de latência de spawn.
- **Sucesso:** o lead escreve o seam `sys` e o GUARANTEES de kill/host sem nenhuma dúvida aberta.
- **Invariantes:** toda afirmação tem execução em CI no OS correspondente; o código do spike é descartável.
- **Qualidade:** cada ADR traz decisão, alternativas rejeitadas, evidência e consequência para o contrato.
- **DoD:** o Codex confirma que a evidência sustenta a decisão; o lead abre 3 links de CI.
- **Não fazer:** PTY.

#### S2 · Spike: PTY
- **Agente:** Opus · **Revisor:** Codex · **Depende:** repo · **Escreve:** `spikes/pty/**` (branch), `docs/adr/0003-pty.md`
- **Completude:** G0-03b:
  1. `portable-pty` vs implementação própria.
  2. Travamento de EOF no ConPTY e versão mínima do Windows.
  3. `\x03` como Ctrl-C.
  4. Resize.
  5. Filho do ConPTY dentro do Job.
  6. Especificação do matcher que remove VT nos testes.
- **Sucesso:** o lead decide o desenho do W12/W12w e o matcher.
- **Invariantes:** toda afirmação tem execução em CI no OS correspondente.
- **Qualidade:** o ADR inclui uma amostra real da saída bruta do ConPTY.
- **DoD:** revisão do Codex no ADR; o lead abre 3 links de CI.
- **Não fazer:** emulador de terminal.

#### S3 · Spike: empacotamento e runtimes (leve, ~1 dia)
- **Agente:** Sonnet · **Revisor:** Codex · **Depende:** repo · **Escreve:** `spikes/packaging/**` (branch), `docs/adr/0004-packaging.md`
- **Completude:** G0-03c:
  1. napi-rs com pacotes por plataforma em Node 22/24, Bun e Deno (e as flags).
  2. Wheels abi3 `py310` para os 5 alvos, incluindo CPython 3.14.
  3. Runners de CI para os 5 alvos.
  4. Um runtime tokio por processo e a ponte com asyncio.
  5. Passo a passo de trusted publishing.
  6. Tamanhos e tempos de instalação.
- **Sucesso:** snippets de CI prontos para o W14 e a lista de dependências fechada.
- **Invariantes:** nada é publicado em registry.
- **Qualidade:** cada alvo com um link de CI de instalação limpa.
- **DoD:** revisão do Codex no ADR; o lead instala localmente o wheel e o pacote npm de macOS-x64.
- **Não fazer:** o binding real.

#### WG0 · Gate G0
- **Quem:** o lead redige, você assina · **Depende:** R1, R2, S1–S3 · **Escreve:** `docs/decisions/G0.md`
- **Completude:** G0-04.
- **Sucesso:** decisão explícita:
  - alternativa cobre ≥80% → parar;
  - ninguém trocaria → parar;
  - seguir só com dor confirmada em ≥2 ecossistemas.
- **Invariantes:** nenhum WP da Fase 1 é despachado antes da sua assinatura.
- **Qualidade:** cada critério cita a evidência.
- **DoD:** assinado e commitado no B0, junto com a lista de mudanças de contrato que vieram dos ADRs.
- **Não fazer:** "seguir com ressalvas" sem waiver.

### Fase 1: v0.1 (`exec` + `pty`, 3 linguagens)

#### W00 · Scaffold + contrato congelado (lead, não delegado)
- **Quem:** lead · **Depende:** G0 e D5 · **Escreve:**
  - `Cargo.toml` (`members = ["crates/*", "bindings/*"]`), `rust-toolchain.toml`;
  - `AGENTS.md` e `CLAUDE.md`;
  - `GUARANTEES.md`, `docs/glossary.md`, `conformance/SPEC.md`, `conformance/FIXTURE.md`;
  - `crates/hugr-omni/src/{lib.rs, api/**, error/mod.rs}` (API pública documentada, corpos stub) e a fachada `mod.rs` de cada módulo (`spawn`, `process`, `io`, `pty`, `sys`): os seams congelados;
  - `bindings/node/{index.d.ts,package.json}`, `bindings/python/{pyproject.toml,python/hugr_omni/*.pyi}`;
  - `.github/workflows/{core,windows}.yml`, `.github/review/schema.json`.
- **Completude:** SCF-01 (compila nos 3 OS; os stubs retornam erro, nunca panic; `file-size-guard` e seus testes de dentes registrados no `core.yml`).
- **Sucesso:** qualquer WP pode ser despachado sem nenhuma pergunta de interface.
- **Invariantes:** contrato mínimo; DSL de cenários com ≤10 passos; hash do contrato registrado.
- **Qualidade:** doc pública escrita primeiro (README-driven); os quickstarts compilam.
- **DoD:** o Codex pergunta "isto é o mínimo? algum nome confunde?"; `move-in` confere o relay hook.
- **Não fazer:** implementar comportamento.

#### W01 · Fixture + contrato (cenários + runner Rust)
- **Agente:** Opus · **Revisor:** Codex (vacuidade) · **Depende:** W00 · **Escreve:** `crates/omni-fixture/**`, `conformance/scenarios/**`, `crates/hugr-omni/tests/**`
- **Completude:** FIX-01 (os subcomandos do `FIXTURE.md`); ACC-01 (cada item do Apêndice A com um cenário vermelho, mais as suítes dos seams `sys`/`pty`).
- **Sucesso:** a suíte mostra o produto inteiro "faltando", promessa por promessa, em ~35 cenários.
- **Invariantes:** cada cenário falha pelo motivo certo; o fixture sincroniza por marcadores (`READY`); nenhum cenário além do Apêndice A.
- **Qualidade:** o nome do cenário começa pelo ID do item; o runner Rust tem ≤ ~300 linhas.
- **DoD:** baseline vermelho capturado; o Codex confirma, item a item, que o cenário pega uma implementação errada plausível.
- **Não fazer:** inflar a suíte com casos que o QA cobre melhor.

#### W02 · Runners TS e Python
- **Agente:** Sonnet · **Revisor:** Codex (vacuidade) · **Depende:** W00, W01 · **Escreve:** `bindings/node/test/**`, `bindings/python/tests/**`
- **Completude:** ACC-02: runner TS sem framework (Node/Bun/Deno) e os testes do item de idioma C-TS-01 (vermelhos; quem os torna verdes é o W13). O runner pytest e o C-PY-01 entram no v0.2.
- **Sucesso:** o mesmo `scenarios/*.json` roda nos 5 runtimes, e o número de cenários executados bate com o total.
- **Invariantes:** o runner só lê os cenários; `skipped` conta como falha.
- **Qualidade:** cada runner com ≤ ~300 linhas; a falha mostra cenário, passo e esperado vs obtido.
- **DoD:** tudo vermelho contra os stubs, pelo motivo certo.
- **Não fazer:** criar cenário.

#### W03 · Resolução, ambiente e erros
- **Agente:** Opus · **Depende:** W01 · **Escreve:** `crates/hugr-omni/src/spawn/{resolve,env,validate}.rs`, `crates/hugr-omni/src/error/messages.rs`
- **Completude:** C-SPAWN-01, C-SPAWN-03, C-ENV-01, C-ERR-01, C-ERR-02.
- **Sucesso:** `run("npm", …)` funciona igual nos 3 OS, e cada erro diz como consertar.
- **Invariantes:**
  - a busca usa o PATH final do filho;
  - chaves de env case-insensitive no Windows;
  - validação antes de qualquer syscall;
  - a resolução nunca executa nada.
- **Qualidade:** funções puras (sem spawn) para resolve e env; o lead lê todas as mensagens de erro.
- **DoD:** mutation probe do lead removendo o PATHEXT.
- **Não fazer:** cache de resolução; expansão de variáveis.

#### W05 · `sys/unix`
- **Agente:** Opus · **Depende:** W01 · **Escreve:** `crates/hugr-omni/src/sys/unix/**`
- **Completude:** SYS-U.
- **Sucesso:** o W07 mata árvores no Linux e no macOS sem nenhum `cfg` fora deste arquivo.
- **Invariantes:** toda syscall checada; nunca sinalizar um grupo já colhido; `CLOEXEC` em tudo.
- **Qualidade:** `// SAFETY:` em todo `unsafe`.
- **DoD:** verde no macOS nativo e no Linux via Docker.
- **Não fazer:** graça, timeout, PTY.

#### W06 · `sys/windows`
- **Agente:** Opus · **Depende:** W01 · **Escreve:** `crates/hugr-omni/src/sys/windows/**`
- **Completude:** SYS-W, C-SPAWN-02.
- **Sucesso:** argumentos chegam idênticos; `.bat` nunca vira injeção; o filho nasce dentro do Job.
- **Invariantes:** sem janela de corrida na criação; quoting recusa o que não é representável; handles fechados em todos os caminhos.
- **Qualidade:** o algoritmo de quoting cita a fonte e tem testes de tabela.
- **DoD:** verde no `windows.yml` sob demanda; o Codex tenta achar uma entrada que vire comando.
- **Não fazer:** ConPTY.

#### W07 · Child: kill de árvore e saída
- **Agente:** Opus · **Depende:** W05, W06 · **Escreve:** `crates/hugr-omni/src/process/{tree,exit}.rs`
- **Completude:** C-KILL-01, C-KILL-02, C-KILL-03, C-EXIT-01, C-SCOPE-01.
- **Sucesso:** depois de `kill()` nada sobra, mesmo que a raiz já tenha morrido e só restem netos.
- **Invariantes:** `kill` idempotente; `drop` não bloqueia; `wait` tem uma fonte única de verdade; `Exit` vem de uma função pura.
- **Qualidade:** zero `cfg`; a precedência de `reason` documentada no código.
- **DoD:** QA rápido local com K1 = 0; mutation probe do lead removendo a etapa de força.
- **Não fazer:** timeout e cancelamento.

#### W09 · Supervisão: timeout, cancelamento, host
- **Agente:** Opus · **Depende:** W07 · **Escreve:** `crates/hugr-omni/src/process/{supervise,registry}.rs`
- **Completude:** C-TMO-01, C-TMO-02, C-HOST-01 (host Rust), C-RS-01, C-RS-02 (todos os cenários via API Rust; a prova nos 5 alvos vem no CI completo do B2).
- **Sucesso:** timeout e cancelamento nunca deixam nada para trás; o host sair limpa tudo, no tier declarado.
- **Invariantes:** cada waiter resolve exatamente uma vez; o registry tem só `register`/`unregister`/`kill_all`; nada bloqueia o executor.
- **Qualidade:** uma máquina de estados explícita, sem flags espalhadas.
- **DoD:** QA-E local (200 comandos com cancelamentos) com K1 = K2 = 0; o Codex revisa corridas.
- **Não fazer:** tratar sinais do host além do que o ADR-0002 decidir.

#### W10 · IO + `run()`
- **Agente:** Opus · **Depende:** W01 (o `run` integra com W07) · **Escreve:** `crates/hugr-omni/src/io/{out,decode,stdin,collect}.rs`
- **Completude:** C-IO-01, C-IO-02, C-IO-03, C-IO-04, C-RUN-01.
- **Sucesso:** ler só o começo da saída de um dev server não o congela; `run()` resolve o caso de 80% numa linha.
- **Invariantes:**
  - buffer de 1 MiB sem consumidor, com descarte contado;
  - backpressure sem perda quando há consumidor;
  - UTF-8 com estado entre chunks;
  - EPIPE vira `CLOSED`;
  - nada se perde no exit.
- **Qualidade:** `run` é composição de `spawn` + coleta (≤ ~150 linhas).
- **DoD:** QA-D local (flood, stdin, pipe herdado) com K2 = K5 = 0.
- **Não fazer:** parsing de linhas; strip de ANSI.

#### W12 · PTY: Unix + integração
- **Agente:** Opus · **Depende:** W05, W07, W10, ADR-0003 · **Escreve:** `crates/hugr-omni/src/pty/session.rs`, `crates/hugr-omni/src/pty/unix/**`
- **Completude:** PTYSYS-U, C-PTY-01, C-PTY-02, C-PTY-03, C-PTY-04.
- **Sucesso:** bash/python interativo, resize, Ctrl-C e saída sem perda, igual em 3 OS (com o W12w).
- **Invariantes:** o PTY usa o mesmo modelo de grupo/Job e o mesmo `io`; falha de setup não vaza nada.
- **Qualidade:** `session.rs` sem `cfg`, com ≤ ~200 linhas.
- **DoD:** QA-C local (bash, python, node) com K2 = 0.
- **Não fazer:** emulador de terminal.

#### W12w · ConPTY
- **Agente:** Opus · **Depende:** W06, ADR-0003 · **Escreve:** `crates/hugr-omni/src/pty/windows/**`
- **Completude:** PTYSYS-W.
- **Sucesso:** o travamento clássico do ConPTY (o filho sai e a leitura nunca termina) não acontece em 200 execuções.
- **Invariantes:** o filho nasce no Job; o fechamento segue a ordem do ADR-0003.
- **Qualidade:** o caminho do ConPTY documentado passo a passo no código.
- **DoD:** 200 execuções verdes no `windows.yml`; o Codex revisa a ordem de fechamento de handles.
- **Não fazer:** Unix.

#### W13 · Binding Node/Bun/Deno
- **Agente:** Opus · **Depende:** B2, B3 · **Escreve:** `bindings/node/{src,lib}/**` (Rust dividido em `child`, `run`, `error`, `convert`), `bindings/node/index.js`
- **Completude:** C-TS-01 (idiomas e host TS), C-TS-02 (todos os cenários via TS em Node 22/24, Bun e Deno).
- **Sucesso:** os quickstarts TS da seção 3 rodam como estão escritos, nos 3 runtimes.
- **Invariantes:** zero lógica de processo em JS; o GC nunca mata o filho; hook de saída do host conforme o ADR-0002.
- **Qualidade:** wrapper JS com ≤ ~200 linhas; `index.d.ts` intocado; zero `any`.
- **DoD:** QA-A, QA-B e QA-E em Node locais com K1 = K2 = 0; o Codex revisa a fronteira FFI.
- **Não fazer:** mexer em `package.json` (é do W14).

#### W15 · Binding Python (v0.2)
- **Agente:** Opus · **Depende:** B2, B3 · **Escreve:** `bindings/python/src/**` (Rust dividido em `child`, `run`, `error`, `convert`), `bindings/python/python/hugr_omni/{__init__,aio}.py`
- **Completude:** C-PY-01 (idiomas e host Python), C-PY-02 (todos os cenários via Python sync e aio, em 3.10 e 3.14).
- **Sucesso:** os quickstarts Python rodam como estão escritos.
- **Invariantes:** o GIL é solto em toda espera; `KeyboardInterrupt` e cancelamento matam a árvore e propagam; `.pyi` intocados.
- **Qualidade:** wrapper com ≤ ~200 linhas; `pyright --strict` limpo.
- **DoD:** QA-A, QA-B (Flask com reloader) e QA-E em Python locais com K1 = K2 = 0; o Codex revisa GIL e FFI.
- **Não fazer:** mexer em `pyproject.toml`.

#### W14 · Empacotamento (npm, PyPI, crates.io)
- **Agente:** Sonnet · **Depende:** W00, ADR-0004 · **Escreve:** `bindings/node/{package.json,npm/**}`, `bindings/python/pyproject.toml`, `crates/hugr-omni/examples/**`, `.github/workflows/pkg.yml`
- **Completude:** C-PKG-01 — no v0.1, só npm (Node/Bun/Deno); PyPI e crates.io entram no v0.2.
- **Sucesso:** K9 = 100%: instalação limpa nos 5 alvos × npm/bun/deno (v0.1), hello em < 30 s. Snippets prontos no ADR-0004.
- **Invariantes:** nenhum `postinstall` que compile ou baixe; nenhuma sdist que compile de surpresa.
- **Qualidade:** mensagem clara para plataforma não suportada; metadados completos nos 3 registries.
- **DoD:** `npm pack`, wheel e `cargo publish --dry-run` instalados em runner limpo (no CI completo do B4).
- **Não fazer:** publicar.

#### W18 · Docs + checks de paridade e garantias
- **Agente:** Sonnet · **Depende:** B4 · **Escreve:** `README.md`, `*/README.md`, `docs/guide/**`, `scripts/{readme-check,surface-check,guarantees-check}/**`, `.github/workflows/docs.yml`
- **Completude:** C-DOC-01, C-PAR-01, C-GUA-01, C-ARC-01.
- **Sucesso:** um dev entende e usa em 5 minutos; o README abre com TS e mostra os números do QA contra o stdlib.
- **Invariantes:** todo bloco de código roda no CI; um check que não consegue ler sua entrada falha alto.
- **Qualidade:** guia "receitas para agentes" (dev server, testes com timeout, terminal, cleanup); cada check com um teste de dentes.
- **DoD:** o Codex lê como usuário novo e lista o que confundiu; o lead injeta uma violação de paridade e vê o check falhar.
- **Não fazer:** site de docs.

#### Q1 · Harness de QA + KPIs
- **Agente:** Opus · **Depende:** W00 (o harness começa contra o stdlib e liga no omni quando o B2 entra) · **Escreve:** `qa/**`, `.github/workflows/qa.yml`
- **Completude:** QA-01: workloads QA-A..E rodando nas 3 linguagens, com omni e com o baseline stdlib; relatório K1–K8 por OS em JSON + Markdown.
- **Sucesso:** um comando (`qa/run --quick` ou `--full`) devolve a tabela de KPIs, omni vs stdlib, em macOS, Linux (Docker) e Windows (CI).
- **Invariantes:** a contagem de órfãos é medida pelo OS (árvore de processos real), nunca inferida; os repos dos workloads são fixados por commit e rodam sem rede.
- **Qualidade:** quick ≤ 5 min local; full ≤ 30 min por OS; um relatório legível por humano.
- **DoD:** o lead roda o quick, injeta um órfão de propósito e vê K1 falhar (teste de dentes).
- **Não fazer:** dashboard; histórico em banco.

#### Q2 · Usuário frio (K10)
- **Quem:** o lead despacha agentes Sonnet novos, só com o README (v0.1: TS; v0.2: Python e Rust) · **Depende:** W18 · **Escreve:** `docs/ux/**`
- **Completude:** UX-01.
- **Sucesso:** 15/15 tarefas; mediana até o primeiro sucesso ≤ 5 min. Cada atrito vira um WP de correção, e o teste roda de novo com agentes novos.
- **Invariantes:** os agentes nunca veem código, plano ou conversa.
- **Qualidade:** cada atrito classificado (bloqueante, incômodo, nenhum), com a transcrição resumida.
- **DoD:** você assina o relatório.
- **Não fazer:** dar dicas além do README.

#### W21 · Release v0.1
- **Quem:** o lead prepara, **você aprova a publicação** · **Depende:** B5 verde, KPIs no alvo, trusted publishing, REL-02 · **Escreve:** `.github/workflows/release.yml`, `CHANGELOG.md`, `RELEASING.md`
- **Completude:** C-REL-01.
- **Sucesso:** uma tag publica as 3 linguagens na mesma versão; smoke pós-publicação verde nos 5 alvos.
- **Invariantes:** nenhum token de longa duração; release reproduzível a partir da tag.
- **Qualidade:** CHANGELOG escrito para usuários.
- **DoD:** passe de risco do lead (o que é mock, o que depende de humano, qual o pior caso) e QA full com todos os KPIs no alvo; sua aprovação antes do push da tag.
- **Não fazer:** publicar sem sua aprovação na hora.

### Fases 2–3: sandbox (detalhes re-planejados depois do S4)

#### S4 · Spike: sandbox macOS/Linux + threat model
- **Agente:** Opus · **Revisor:** Codex (red team) · **Depende:** v0.1 · **Escreve:** `spikes/sandbox/**` (branch), `docs/adr/0005-sandbox.md`
- **Completude:** SBX-00:
  - Seatbelt (incluindo o status de deprecated do `sandbox-exec`);
  - Landlock vs namespaces vs bubblewrap, com as restrições de distro;
  - caminhos de sistema sempre legíveis;
  - threat model, com o que está fora do escopo.
- **Sucesso:** o lead congela o contrato `sandbox` sem nenhuma dúvida.
- **Invariantes:** fail-closed desde o desenho.
- **Qualidade:** threat model com exclusões explícitas.
- **DoD:** o Codex tenta derrubar o threat model até ficar sem achados.
- **Não fazer:** Windows.

#### SB1 · Política, plumbing e bindings
- **Agente:** Sonnet · **Depende:** S4 · **Escreve:** `crates/hugr-omni/src/sandbox/{mod,policy}.rs` (fachada + política), `bindings/*/src/sandbox.rs`
- **Completude:** C-SBX-01.
- **Sucesso:** o dev escreve a política uma vez e ela vale igual com ou sem PTY, nas 3 linguagens.
- **Invariantes:** fail-closed absoluto; política inválida nunca vira política mais fraca.
- **Qualidade:** a política é um tipo de dados simples, sem DSL.
- **DoD:** mutation probe removendo a checagem de disponibilidade.
- **Não fazer:** regras por syscall ou por porta.

#### SB2 · Backend macOS
- **Agente:** Opus · **Depende:** SB1 · **Escreve:** `crates/hugr-omni/src/sandbox/macos.rs`
- **Completude:** C-SBX-02.
- **Sucesso:** um agente no macOS não escreve fora do repo nem acessa a rede quando a política proíbe, e o permitido funciona.
- **Invariantes:** o perfil é gerado com escaping correto; nenhum caminho do usuário entra cru no perfil.
- **Qualidade:** perfil legível.
- **DoD:** o Codex procura injeção no perfil.
- **Não fazer:** expor Seatbelt cru na API.

#### SB3 · Backend Linux
- **Agente:** Opus · **Depende:** SB1 · **Escreve:** `crates/hugr-omni/src/sandbox/linux/**`
- **Completude:** C-SBX-03.
- **Sucesso:** a mesma política dá o mesmo resultado que no macOS.
- **Invariantes:** detecção de suporte antes do spawn; se faltar algo, `SANDBOX_UNAVAILABLE`.
- **Qualidade:** o CI inclui uma distro com restrição de user namespaces.
- **DoD:** o Codex revisa a ordem de aplicação das restrições.
- **Não fazer:** binário externo sem decisão no ADR.

#### SB4 · Suíte de fuga independente
- **Agente:** Opus, que **não** implementou backend nenhum, + Codex como red team · **Depende:** SB2, SB3 · **Escreve:** `crates/hugr-omni/tests/sandbox_escape/**`
- **Completude:** C-SBX-04.
- **Sucesso:** toda tentativa das categorias do threat model é bloqueada.
- **Invariantes:** a suíte é escrita a partir do threat model, sem ler os backends.
- **Qualidade:** cada tentativa documenta o que tentou e por que deveria falhar.
- **DoD:** o Codex propõe tentativas extras até ficar sem ideias.
- **Não fazer:** corrigir backend.

#### SB6 · Windows (spike S5 + backend)
- **Agente:** Opus · **Depende:** SB1 · **Escreve:** `crates/hugr-omni/src/sandbox/windows/**`, `docs/adr/0006-sandbox-windows.md`
- **Completude:** C-SBX-05.
- **Sucesso:** o GUARANTEES mostra exatamente o que o Windows bloqueia, e cada linha tem teste.
- **Invariantes:** o que não for garantido vira `SANDBOX_UNAVAILABLE`.
- **Qualidade:** o tier é explicado em linguagem de usuário.
- **DoD:** começa pelo spike (decisão de seguir ou não registrada no ADR); Codex como red team.
- **Não fazer:** prometer paridade com macOS/Linux.

---

## 8. Status (âncora viva)

| WP | Estado | Branch / PR | Notas |
|---|---|---|---|
| R1 | concluído | `docs/research/landscape.md` | processkit 81,8% no papel |
| R2 | concluído + verificado | `docs/research/pain.md` | dor nos 3 ecossistemas (teto, não medida) |
| P0 | concluído | `docs/research/processkit-fit.md` | 1 de 24 itens como está |
| E1 | concluído (macOS + Linux parcial) | `spike/processkit-eval` | Windows não medido (billing) |
| Codex | concluído | `docs/research/processkit-audit.md` | "build on it with fixes" |
| S3 | concluído (local) | `spike/packaging` · ADR-0004 | Q8: wait do processkit trava sob Node/Bun no Linux sem pidfd |
| WG0 | **assinado: B** | `docs/decisions/G0.md` | TS no v0.1; Python e Rust no v0.2 |
| S1, S2 | pausados | `spike/process`, `spike/pty` (só local) | retomam quando o billing destravar (precisam de Windows) |
| B0 | montando | `bundle/B0` | pesquisa + ADR-0004 + G0; falta Codex conferir citações do R1 |
| W00 | aguardando | — | depende de D5 (API) e D11 (billing) |
---

## 9. Riscos

| Risco | Resposta |
|---|---|
| Alternativa já existe (R1 ≥ 80%) | parar no G0 e contribuir com ela |
| Gracioso fraco no Windows | tier declarado; a força após `graceMs` continua garantida |
| Host morto à força no macOS/Linux | declarado no GUARANTEES; nada de prometer o que o OS não dá |
| ConPTY trava no EOF | W12w tem meta de 200/200; sem isso não há release |
| Falha só no Windows descoberta tarde (bundle) | `windows.yml` sob demanda para WPs com muito Windows; atribuição pelo ID do item |
| Fila de revisão do lead | máximo de 3 WPs esperando; parar de despachar até baixar |

---

## Apêndice A · Contrato (~35 itens) e itens de gestão

Ver `docs/acceptance.md`.

## Apêndice B · Layout do repo

```text
Cargo.toml  AGENTS.md  CLAUDE.md  GUARANTEES.md  PLAN.md
conformance/{SPEC.md, FIXTURE.md, scenarios/*.json}
crates/hugr-omni/src/{lib.rs, api/, error/, spawn/, process/, io/, pty/{session.rs,unix/,windows/}, sys/{unix/,windows/}, sandbox/}
crates/hugr-omni/{tests/, examples/}
crates/omni-fixture/
bindings/node/{src/, lib/, index.d.ts, npm/, test/}
bindings/python/{src/, python/hugr_omni/, tests/}
qa/  scripts/  docs/{adr,research,decisions,guide,ux,partners}/  .github/{workflows,review}/
```

## Apêndice C · Template do brief

```text
RELAY-ARM:<token>                      (se o hook do relay estiver ligado)
WP <id> — <título>                     modelo: <opus|sonnet>
STEP 0  EXPECTED_BASELINE=<sha>; aborte se o HEAD não descender dele. Ecoe BASELINE_VERIFIED.
BRANCH  wp/<id> (já criada na sua worktree). Push no primeiro commit que compila.
ALVO    arquivos que você ESCREVE (só estes): <lista>
LEIA    (nesta ordem, nada além): <caminhos + linhas>
CONTRATO <trecho congelado, verbatim>
CARD    <o card inteiro do WP, verbatim: Completude, Sucesso, Invariantes, Qualidade, DoD, Não fazer>
ITENS   <id — promessa — cenário> (vermelho agora; faça ficar verde; testes são somente-leitura)
ARMADILHAS nunca `git add -A`; stage por nome; `git diff --name-only <baseline>...HEAD` antes de cada push;
           dependência nova = pare e pergunte; não abra PR.
GATES   python3 scripts/file-size-guard.py · cargo fmt --check · cargo clippy --all-targets -- -D warnings ·
        cargo clippy --target x86_64-pc-windows-msvc -- -D warnings · cargo test -p hugr-omni <filtro> ·
        scripts/linux-docker cargo test -p hugr-omni <filtro> · <gates do card, ex.: qa/run --quick>
PARE    em "branch wp/<id> empurrada, verde localmente, aguardando o lead". Você não integra nem abre PR.
RETORNO (exatamente isto):
  WP <id> · branch wp/<id> · head <sha> · BASELINE_VERIFIED <sha>
  ITENS    <id> verde [macos|linux|win-compile] ...
  GATES    <cmd> → <última linha>
  KPI      <se o card pede QA: K1/K2/K5 = …>
  ARQUIVOS <lista>
  ERREI NO BRIEF? <1–3 linhas>
```

## Apêndice D · Revisão Codex contra o card

```bash
codex exec -s read-only -C "$WORKTREE" --ephemeral \
  --output-schema .github/review/schema.json -o "$REVIEWS/$WP-r$N.json" - < "$REVIEWS/$WP-prompt.md"
```

Prompt (gerado pelo lead por WP e por rodada):

```text
ROLE: independent, adversarial reviewer for hugr-omni. You did not write this change; assume it is wrong
until the diff and its evidence prove otherwise.
SCOPE: `git diff <baseline>...HEAD` in this worktree. Read-only commands only.
THE CARD (the axioms this work is judged against, verbatim): <card do WP>
GLOBAL INVARIANTS / QUALITY: <seções 4.3 e 4.4 verbatim>   GLOSSARY: <docs/glossary.md>
FOR EACH CARD FIELD return pass|fail with concrete evidence (file:line or command output):
  completude  — every owned item is implemented and its scenario genuinely exercises it
  sucesso     — the user-visible outcome the card promises is actually delivered
  invariantes — every card invariant and global INV holds, including on error paths
  qualidade   — card quality bar + QS; flag over-engineering, tests without value, any code file > 600 lines
                (P2: needs a split plan; > 650 is P0) and any import that points up the module layering
  dod         — every DoD bullet of the card is satisfied or demonstrably satisfiable
Then list findings. SEVERITY: P0 wrong behavior/security/data loss/invariant broken/contract test edited;
P1 vacuous scenario, flaky, UX or message regression, wrong docs; P2 clarity/over-engineering; P3 nit.
Never invent filler: empty findings are fine.
```

Schema:

```json
{ "type": "object", "additionalProperties": false,
  "required": ["card", "findings", "verdict"],
  "properties": {
    "card": { "type": "object", "additionalProperties": false,
      "required": ["completude", "sucesso", "invariantes", "qualidade", "dod"],
      "properties": {
        "completude":  { "$ref": "#/$defs/field" }, "sucesso": { "$ref": "#/$defs/field" },
        "invariantes": { "$ref": "#/$defs/field" }, "qualidade": { "$ref": "#/$defs/field" },
        "dod":         { "$ref": "#/$defs/field" } } },
    "findings": { "type": "array", "items": { "type": "object", "additionalProperties": false,
      "required": ["severity", "file", "line", "problem", "fix"],
      "properties": { "severity": { "enum": ["P0", "P1", "P2", "P3"] }, "file": { "type": "string" },
        "line": { "type": "integer" }, "problem": { "type": "string" }, "fix": { "type": "string" } } } },
    "verdict": { "enum": ["approve", "changes_requested"] } },
  "$defs": { "field": { "type": "object", "additionalProperties": false, "required": ["status", "evidence"],
    "properties": { "status": { "enum": ["pass", "fail"] }, "evidence": { "type": "string" } } } } }
```

Merge no bundle só com os 5 campos em `pass`, zero P0/P1 e a verificação do lead.

## Apêndice E · Registro de decisões

- 2026-10-01 · Owner: testes determinísticos se limitam ao contrato público (~35 itens); o peso da prova vai para o QA de uso real com KPIs. O loop do crítico frio da suíte foi encerrado na rodada 8 por essa diretriz. Os achados finos que restaram foram absorvidos como KPIs (K1–K6) ou como linhas de item existente (códigos de saída > 255 no Windows; paridade no nível de opção e campo).
- 2026-10-01 · Owner: PRs em bundle por onda; CI completo uma vez por bundle; verificação local primeiro.
- 2026-10-01 · Owner: repo `HuGR-Labs/hugr-omni`, público; nome `hugr-omni`.
- 2026-10-01 · R1 encontrou o `processkit` (Rust 3.3.4 + processkit-py 1.5.0, MIT), com 81,8% de cobertura nos 3 OS em 2 linguagens, o que dispara o nosso critério de parada. Owner: **pivotar** para hugr-omni = pacote TypeScript (Node/Bun/Deno) sobre o processkit, mais a camada de sandbox depois. **Condição do Owner:** não confiar no README; o pivô só se confirma com a avaliação prática (E1, KPIs nos 3 OS) e a auditoria independente do código (Codex). Até lá, S1/S2 ficam pausados. Sinais medidos no fonte v3.3.4: `src` com ~86 mil linhas em 58 arquivos (28 acima de 650 linhas), 308 ocorrências de `unsafe`, CI em 5 SOs, criado em 2026-05-31, 55 versões, um autor principal.
- 2026-10-01 · **G0 assinado: opção B.** A evidência medida (fit: 1 de 24 itens como está; perda silenciosa de saída; travamento sob Node/Bun no Linux sem pidfd; churn alto) mostrou que construir em cima do processkit nos faria reescrever I/O, timers, motivos, saída do host e o wait, mantendo uma dependência de 86 mil linhas. O processkit fica como **referência** (MIT): reaproveitamos técnicas (filho suspenso → Job → resume; cgroup v2 quando delegado), sem dependência de código. Linguagens: TS no v0.1; Python e Rust no v0.2. S1/S2 retomam assim que o billing do Actions for destravado (precisam de Windows).
- 2026-10-01 · Owner: monolito modular + god-file guard. Limites por arquivo de código (não por PR): ideal 400, ok 600, máximo 650; documentos fora. Implementado em `scripts/file-size-guard.py`, com testes de dentes em `scripts/test_file_size_guard.py` (9 casos) e mutation probe no repo real (um arquivo de 651 linhas → FAIL; removido → verde).
