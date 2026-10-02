# HANDOFF — estado do hugr-omni (2026-10-02, tarde)

Arquivo para retomar o trabalho depois de uma compactação de contexto ou de uma sessão que caiu. A fonte da verdade é
o `PLAN.md` (status na seção 8, decisões no Apêndice E). Aqui está só o "onde parei".

## Papéis

- **Usuário = stakeholder.** O lead (sessão principal do Claude) aprova as decisões técnicas e cuida de merge, push,
  PRs, manutenção e higiene do repo. Ao usuário, perguntar só sobre:
  - publicar pacotes;
  - mensagens em nome dele;
  - waivers de rigor;
  - metas de KPI.
- **Quem faz o quê:**
  - execução: sub-agentes Claude Opus, um por worktree (`.claude/worktrees/agent-*`, ignorado pelo git), branch
    `wp/<id>`;
  - revisão: Codex CLI contra o card do WP;
  - brief e prompt da revisão: `scripts/brief.py` e `scripts/review-prompt.py`, gerados do card vivo no PLAN.
- **Credenciais:** `gusmhs` é a única conta no `gh` (a `gmhelmold` foi suspensa em 2026-10-02). Push com `git push`;
  gh com `-R gusmhs/hugr-omni`. Nunca trocar a conta nem mexer em config global.

## Repo `gusmhs/hugr-omni`

- **`main` = `ee40167`:** B0 + B1a (PR #2), com o contrato congelado, o ADR-0005 e o scaffold W00 completo:
  - workspace de 4 crates;
  - seams congelados;
  - protocolo v1 com o codec testado;
  - SPEC e FIXTURE;
  - GUARANTEES, AGENTS.md;
  - CI `core.yml` (verde nos 3 OS + musl estático) e `windows.yml` sob demanda.
- **`bundle/B2`:** a partir da `main`, já tem o `.gitignore` do `.claude/`, o seam `binding` (W03) e o contrato
  recusando caminhos do Windows relativos ao drive. É aqui que a primeira onda é integrada.
- **Spikes:** os branches `spike/*` ficam só no GitHub (as evidências dos ADRs apontam para eles); as worktrees locais
  foram removidas.

## Estado

- **`main` = `70d1828`:** primeira onda completa (W00, W01, W03–W06), merged pelo PR #4 com CI verde (3 OS, supervisor
  no musl, K4 em release no Linux). Linha do contrato: `0 passed, 36 pending, 0 failed`.
- **`bundle/B3`** (a partir da `main`): seam interno do `io` congelado (`Pumps`, `Source`, `Collected`, `Stdin`),
  `scripts/brief.py` (gera o brief a partir do card) e `scripts/review-prompt.py` (o prompt do Codex).
- **Segunda onda em execução,** baseline `a62ddf8`, briefs em `~/Documents/HuGR/hugr-omni-reviews/briefs/`:
  - W10, io (Opus);
  - W07, Child (Opus);
  - W02, runner TS (Sonnet);
  - W12w, ConPTY (Opus).
- **Depois:**
  - W09 (timeout/cancel/run), depois do W07;
  - W12 (PTY Unix + lado host), depois do W10, com o seam do `pty` congelado no V1;
  - W13 (binding Node), depois do B2/B3.
  Os itens saem do `conformance/pending.txt` (só o lead edita) quando ficam verdes.
- **Provas pendentes do lead:** matar o supervisor real durante um flood (DoD do W04), quando o W07 entrar; e, no W07,
  o probe "tirar a etapa forçada do stop".
- **Fluxo de cada entrega:**
  1. L0;
  2. o lead lê o código;
  3. o probe;
  4. o Codex até aprovar;
  5. gates a frio, incluindo macOS sob carga e Linux privilegiado;
  6. squash no bundle.

## Avisos

- Docker Desktop: subir com `open -a Docker` se cair; cada agente usa o próprio volume `omni-target-<WP>`.
- O material para investidores está fora do repo: `~/Documents/HuGR/omni-investidores.html`.

## Estado em 2026-10-02 (fim da sessão, limite de uso)

- `bundle/B3` (`bc08537`+): W12w, W02 e W10 entraram (Codex aprovou os três; probes do lead vermelhos; Windows CI
  verde nos 2 builds para o W12w; Linux Docker verde em `bc08537`).
- W07: aprovado pelo Codex (r2). O agente está juntando a `bundle/B3` na `wp/W07` e adicionando o teste do
  C-SCOPE-01 (drop pela API pública, inclusive após o runtime do chamador fechar). Commit só local, na worktree
  `.claude/worktrees/agent-aee3e7db1154947a3`. Para entrar: push da `wp/W07`; contrato no macOS; probe do lead
  (esvaziar `Child::drop` → C-SCOPE-01 vermelho); squash; tirar do `conformance/pending.txt`: C-IO-02, C-IO-03,
  C-KILL-01, C-KILL-02, C-KILL-03, C-PROC-01.
- Depois: congelar em `process/deadline.rs` o seam `refuse_if_cancelled(opts)` e `arm(inner, timeout, cancel)`, chamados
  em `spawn_pipe`/`spawn_pty`; despachar W09 (deadline.rs, run.rs; `run()` nunca descarta o future do `collect`) e
  W12 (pty_unix, pty, `spawn_pty`/`PtyChild::resize`).
- Os agentes não conseguem dar push (o classificador de permissões bloqueia); o lead faz o push com o "sim" do usuário.
