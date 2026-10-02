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
  - prompt da revisão: `scratchpad/reviews/wp-prompt.py`, regenerável a partir do Apêndice D do PLAN.
- **Credenciais:** a conta ativa do `gh` pode ser outra (`gustavomhss`). Nunca trocar a conta nem mexer em config
  global; usar o token do `gmhelmold` comando a comando:
  - push: `git -c credential.helper= -c credential.helper='!f(){ echo username=gmhelmold; echo "password=$(gh auth token --user gmhelmold)"; }; f' push ...`
  - gh: `GH_TOKEN=$(gh auth token --user gmhelmold) gh ... -R gmhelmold/hugr-omni`

## Repo `gmhelmold/hugr-omni`

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

## Estado (fim do dia 2026-10-02)

- **Primeira onda integrada no `bundle/B2`** (rebaseado sobre a `main`):
  - W01 (fixture + 43 cenários + runner Rust, registro `conformance/pending.txt`);
  - W03 (resolve/env/erros);
  - W04 (cliente/canal);
  - W05 (supervisor Unix);
  - W06 (supervisor Windows).
  Cada um passou pelo Codex até `approve`, com o probe do lead e os gates a frio (macOS sob carga, Linux --privileged, musl, Windows CI).
- **Linha do contrato:** `0 passed, 36 pending, 0 failed`, porque a camada de processo ainda é stub.
- **Próximo:**
  1. PR do B2 → CI completo → merge na `main`;
  2. congelar os seams internos de io/process/pty (V1 do lead);
  3. segunda onda: W10 (io), W07 (Child), W09 (timeout/cancel/run), W02 (runner TS);
  4. depois disso o contrato deve ficar verde, e os itens saem do `pending.txt`.
- **Prova pendente do lead (DoD do W04):** matar o supervisor real durante um flood. Agora é possível, com o W05 integrado.
- **Revisões do Codex:** ficam em `~/Documents/HuGR/hugr-omni-reviews/`. O prompt sai de `scripts/review-prompt.py <WP> <base> <head> [notes]`.

## Avisos

- Docker Desktop: subir com `open -a Docker` se cair; cada agente usa o próprio volume `omni-target-<WP>`.
- O material para investidores está fora do repo: `~/Documents/HuGR/omni-investidores.html`.
