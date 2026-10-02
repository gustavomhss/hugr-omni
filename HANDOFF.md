# HANDOFF — estado do hugr-omni (2026-10-02)

Arquivo para retomar o trabalho depois de uma compactação de contexto. A fonte da verdade continua sendo o
`PLAN.md` (status na seção 8, decisões no Apêndice E); aqui está só o "onde parei".

## Papéis

- **Usuário = stakeholder.** O lead (sessão principal do Claude) aprova as decisões técnicas: contrato, gates,
  merges e go/no-go técnico. Perguntar ao usuário só para: publicar pacotes, mandar mensagem em nome dele,
  waivers de rigor e mudanças de meta de KPI.
- **Execução:** sub-agentes Claude, cada um em worktree própria. **Revisão:** Codex CLI (`codex exec -s read-only
  ... --output-schema`, com `--search` quando precisa de web). Cada WP é julgado contra o próprio card.
- Preferências do usuário: o mínimo de testes determinísticos, QA de uso real com KPIs, PRs em bundle para
  economizar CI, monolito modular com god-file guard (400 ideal / 600 ok / 650 máximo por arquivo de código),
  e nada de overengineering.

## Repo

- `gmhelmold/hugr-omni`, público. Foi transferido da HuGR-Labs, cujo Actions está travado por billing da org.
  O local fica em `~/Documents/HuGR/hugr-omni`.
- Push: `git -c credential.helper= -c credential.helper='!gh auth git-credential' push` (o SSH falha).
- `main` tem o B0 mergeado (PR #1): pesquisa, auditoria e avaliação do processkit, ADR-0004, G0 assinado e
  `scripts/file-size-guard.py`.
- **`bundle/B1`** (atual, já empurrado) tem:
  - `docs/api-contract.md` (CONGELADO);
  - os ADRs 0001–0003 (revisados e substituídos em parte pelo ADR-0005);
  - o **ADR-0005 supervisor (ACEITO, requisitos R1–R10)**;
  - as revisões do Codex em `docs/research/`;
  - o `PLAN.md` refatiado.
- Worktrees dos spikes em `~/Documents/HuGR/hugr-omni-wt/` (process, pty, packaging, pk-eval, supervisor). Todos
  estão empurrados e podem ser removidos quando o B1 entrar. O código dos spikes nunca vai para `main`; o
  `spike/supervisor` (`spikes/supervisor/**`) é a principal referência de código para W04/W05/W06/W12/W12w.

## Decisões fechadas

- **G0 = B:** núcleo próprio enxuto, TypeScript no v0.1, Python e Rust no v0.2. O processkit fica só como
  referência de técnicas.
- **Contrato da API congelado** (`docs/api-contract.md`), depois de 3 rodadas do Codex.
- **ADR-0005:**
  - um supervisor (binário próprio, musl estático no Linux) cria e colhe todos os filhos;
  - o host nunca faz fork e não instala handler de sinal;
  - a sessão é a unidade de kill no Unix;
  - Job + `JOB_LIST` no Windows;
  - o I/O fica no host via fds/handles passados;
  - protocolo: `Spawn`, `Go`, `Signal`, `Stop{grace}`, `Resize`, `Release` / `Ready`, `Spawned`, `SpawnFailed`,
    `Exited`, `Stopped`, `Ack`.
- **K4** = ≤ 1,25× o stdlib ou ≤ +0,3 ms, o que for mais folgado (autorizado pelo stakeholder).
- Testes de spike e de produto precisam **afirmar** o resultado e falhar o CI quando dá errado.

## Próximo passo: W00 (lead, não delegado), no `bundle/B1`

Write-set e completude estão no card W00 do `PLAN.md`. Em resumo:
- workspace com `crates/{hugr-omni, omni-proto, omni-supervisor, omni-fixture}` e `bindings/node`;
- API pública com stubs que retornam erro e nunca dão panic;
- tipos de `omni-proto` congelados;
- fachadas `mod.rs` de todos os módulos;
- `index.d.ts` copiado do contrato;
- `docs/protocol.md` (≤ 1 página, a partir da seção "Evidence (S5)" do ADR-0005);
- `conformance/SPEC.md` (DSL com ≤ 10 passos) e `conformance/FIXTURE.md`;
- `GUARANTEES.md` (linhas por item C-*, com os tiers dos ADRs);
- `AGENTS.md`/`CLAUDE.md` (regras, armadilhas, allowlist de dependências, gates);
- CI `core.yml`: fmt, clippy, testes, file-size guard, build musl estático do supervisor; dispara só em PR
  não-draft para `main`, push em `main` e `workflow_dispatch`;
- `windows.yml` sob demanda;
- `.github/review/schema.json`.

Depois do W00:
1. Revisão do Codex: "isto é o mínimo? algum nome confunde?".
2. Primeira onda, rolling, até 6 agentes:
   - W01: fixture + cenários + runner Rust;
   - W02: runner TS;
   - W03: resolve/env/errors;
   - W04: canal/cliente;
   - W05: supervisor Unix;
   - W06: supervisor Windows.

## Pendências e avisos

- O disco estava com 95% de uso (25 GB livres). Limitar builds Rust locais pesados a 2 agentes por vez.
- O Docker Desktop caiu antes. O E1 deixou volumes Docker `pkeval-target` e `pkeval-cargo` e a imagem `rust:1`,
  que podem ser removidos com o Docker de pé (pedir ok ao usuário).
- O material para investidores está **fora** do repo: `~/Documents/HuGR/omni-investidores.html`.
- Conversas com ≥5 maintainers (G0-05) e design partners (REL-02) são do stakeholder e travam só o release.
