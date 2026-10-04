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
- **Repo e CI (desde 2026-10-04):** GitLab `gmhelmold/hugr-omni` (remote `origin`, ssh; `glab` logado como
  `gmhelmold`). O GitHub (`gusmhs`, remote `github`) ficou só leitura: o Actions foi bloqueado na conta.
  - Os minutos grátis do GitLab acabaram em 2026-10-04 (`ci_quota_exceeded`); o `.gitlab-ci.yml` fica, para quando
    houver minutos. CI ativo: AppVeyor (conta `gustavoschneiter`, `appveyor.yml`: Ubuntu, macOS, Windows Server 2019),
    assim que o usuário autorizar o GitLab no AppVeyor. Até lá: gates locais (macOS nativo, Linux no Docker).
    A key do AppVeyor é do usuário: nunca no repo.
  - Nunca trocar contas nem mexer em config global.

## Repo (histórico do GitHub)

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

## Estado em 2026-10-04

- Repo no GitLab (`gitlab.com/gmhelmold/hugr-omni`), CI nos runners do GitLab (minutos grátis; AppVeyor depois).
- `main` = `c2e745b`: segunda onda inteira, contrato 36/36 (macOS local, Linux CI, Windows 2022 CI menos o W06b).
- Próximo bundle: `bundle/B4` (a partir da `main`).
- Aberto: **W06b** — no runner Windows do GitLab, o teste do W06 `stopped_arrives_only_once..._between_polls` falha
  (CTRL_BREAK não chega; stop de graça 2000 levou 5230 ms). Investigar com pipelines manuais (`TEST_FILTER`).
- Depois: W13 (binding Node/Bun/Deno), W14 (empacotamento), QA, W18, W21.

## Estado em 2026-10-03

- `bundle/B3` tem a segunda onda inteira: W12w, W02, W10, W07 (frios verdes no macOS e no Linux em `3876315`;
  Windows CI verde para o W12w). Ledger: saíram C-IO-02, C-IO-03, C-KILL-01, C-KILL-02, C-KILL-03, C-PROC-01.
- Seam `deadline::{refuse_if_cancelled, arm}` congelado (`2899b13`) e já chamado por `spawn_pipe`/`spawn_pty`.
- Em execução (baseline `409cdac`, só commit local): W09 (deadline.rs, run.rs; C-TMO-01/02, C-HOST-01, C-RS-01/02) e
  W12 (pty_unix, pty, `spawn_pty`/`PtyChild::resize`; PTYSYS-U, C-PTY-01..04). Briefs em
  `~/Documents/HuGR/hugr-omni-reviews/briefs/W09.brief.txt` e `W12.brief.txt`.
- Fluxo por entrega: push da `wp/<id>` pelo lead → L0 → leitura → Codex (`scripts/review-prompt.py`) → probe do lead →
  frios (macOS; Linux no Docker `--privileged`; Windows CI se tocar Windows) → squash na `bundle/B3` → ledger.
- Depois do W09 e do W12: PR da `bundle/B3` para a `main`, CI completo, merge. Então W13 (binding Node).
