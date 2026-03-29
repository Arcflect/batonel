#!/usr/bin/env python3
import yaml
import glob
import os

def generate_prompt(contract_path, prompt_path):
    with open(contract_path, 'r') as f:
        contract = yaml.safe_load(f)

    out = f"# Artifact Prompt: {contract['name']}\n\n"
    out += f"Implement the `{contract['name']}` artifact.\n\n"
    out += f"## Role\n{contract['role']}\n\n"
    out += f"## Module\n{contract['module']}\n\n"

    resps = contract.get('responsibilities', [])
    if resps:
        out += "## Responsibilities\n"
        for r in resps: out += f"- {r}\n"
        out += "\n"

    musts = contract.get('must_not', [])
    if musts:
        out += "## Must not\n"
        for m in musts: out += f"- {m}\n"
        out += "\n"

    ad = contract.get('allowed_dependencies', [])
    if ad:
        out += "## Allowed dependencies\n"
        for d in ad: out += f"- {d}\n"
        out += "\n"

    fd = contract.get('forbidden_dependencies', [])
    if fd:
        out += "## Forbidden dependencies\n"
        for d in fd: out += f"- {d}\n"
        out += "\n"

    inputs = contract.get('inputs', [])
    if inputs:
        out += "## Inputs\n"
        for i in inputs: out += f"- {i}\n"
        out += "\n"

    oups = contract.get('outputs', [])
    if oups:
        out += "## Outputs\n"
        for o in oups: out += f"- {o}\n"
        out += "\n"
        
    crit = contract.get('completion_criteria', [])
    if not crit:
        role = contract['role']
        if role == 'entity':
            crit = ["The entity strictly protects its domain invariants.", "Methods represent business rules, not just generic getters/setters.", "No application, transport, or persistence details leak into this layer."]
        elif role == 'usecase':
            crit = ["The usecase implements exactly one application flow.", "It coordinates domain behavior through ports but does not implement infrastructure natively.", "No HTTP or database logic is present."]
        elif role == 'repository_port':
            crit = ["The abstraction focuses purely on the repository intent (e.g., retrieving aggregates).", "It is fully decoupled from specific SQL, ORM, or database terminology."]
        elif 'handler' in role:
            crit = ["The handler cleanly translates transport models into application requests.", "It invokes the application layer but embeds zero core business rules locally."]
        elif role in ('repository_impl', 'repository'):
            crit = ["The implementation fulfills an outbound port.", "It safely translates between raw persistence data and pure upstream domain models."]
        else:
            crit = ["The artifact focuses exclusively on its defined responsibilities.", "The implementation respects forbidden dependencies and architectural rules."]

    if crit:
        out += "## Completion criteria\n"
        for c in crit: out += f"- {c}\n"
        out += "\n"

    with open(prompt_path, 'w') as f:
        f.write(out)

if __name__ == "__main__":
    count = 0
    for c_path in glob.glob("examples/**/expected/.archflow/contracts/*.contract.yaml", recursive=True):
        p_path = c_path.replace("contracts/", "prompts/").replace(".contract.yaml", ".prompt.md")
        os.makedirs(os.path.dirname(p_path), exist_ok=True)
        generate_prompt(c_path, p_path)
        print(f"Updated {p_path}")
        count += 1
    print(f"Successfully synced {count} prompt files.")
