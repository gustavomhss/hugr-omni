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

## Primeira onda (baseline `ee40167`; W03 rebaseado em `f5d8b89`)

| WP | Branch | Estado |
|---|---|---|
| W01 fixture + cenários + runner Rust | `wp/W01` | agente retomado após a queda da sessão |
| W03 resolve/env/erros | `wp/W03` | entregue; o Codex pediu mudanças (P0: caminho do Windows relativo ao drive; P1: erros de metadata); retrabalho em andamento, junto com o `binding.rs` |
| W04 canal/cliente | `wp/W04` | agente retomado |
| W05 supervisor Unix | `wp/W05` | agente retomado |
| W06 supervisor Windows | `wp/W06` | agente retomado; `windows.yml` disparável |

Fluxo de cada entrega:
1. L0: o HEAD descende do baseline e os arquivos estão dentro do write-set.
2. O lead lê as mensagens e o código.
3. Mutation probe do DoD.
4. Codex contra o card, até aprovar.
5. Squash no `bundle/B2`.

Quando W01/W03/W04/W05/W06 entrarem: PR do B2 → CI → merge. Depois vêm W07/W10 (os seams internos de io e process
são congelados no V1 de cada um), W09, W02 e o resto do DAG.

## Avisos

- Docker Desktop: subir com `open -a Docker` se cair; cada agente usa o próprio volume `omni-target-<WP>`.
- O material para investidores está fora do repo: `~/Documents/HuGR/omni-investidores.html`.
