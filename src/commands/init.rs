use std::fs;
use std::path::Path;

pub fn execute() {
    let files = vec![
        (
            "project.arch.yaml",
            r#"project:
  name: archflow-app
  architecture_style: simple
  language: generic

modules:
  - name: user
    features:
      - create_user
      - user_entity
"#,
        ),
        (
            "placement.rules.yaml",
            r#"roles:
  usecase:
    path: "src/application/usecases/"
  entity:
    path: "src/domain/entities/"
"#,
        ),
        (
            "artifacts.plan.yaml",
            r#"artifacts:
  - name: create_user
    module: user
    role: usecase
    inputs:
      - CreateUserCommand
    outputs:
      - CreateUserResult

  - name: user
    module: user
    role: entity
    outputs:
      - User
"#,
        ),
        (
            "contracts.template.yaml",
            r#"role_templates:
  usecase:
    responsibilities:
      - "Execute one application use case"
      - "Coordinate domain behavior"
    must_not:
      - "Access infrastructure details directly"
      - "Return transport-specific responses"
    implementation_size: "small"

  entity:
    responsibilities:
      - "Represent a core business concept"
      - "Protect domain invariants"
    must_not:
      - "Depend on transport or persistence details"
    implementation_size: "small"
"#,
        ),
    ];

    println!("Archflow Initialization");
    println!("=======================");

    let mut generated_count = 0;

    for (filename, content) in files {
        let path = Path::new(filename);
        if path.exists() {
            println!("  [~] {} already exists, skipping.", filename);
        } else {
            match fs::write(path, content) {
                Ok(_) => {
                    println!("  [+] Generated {}", filename);
                    generated_count += 1;
                }
                Err(e) => {
                    eprintln!("  [!] Failed to generate {}: {}", filename, e);
                    std::process::exit(1);
                }
            }
        }
    }

    println!();
    if generated_count > 0 {
        println!("Initialization complete! Explore your configuration files, then run:");
        println!("  archflow plan");
        println!("  archflow scaffold");
    } else {
        println!("Initialization finished. No new configuration files were generated.");
    }
}
