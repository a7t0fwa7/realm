use starlark::{values::starlark_value_as_type::StarlarkValueAsType, environment::GlobalsBuilder};
use starlark_derive::starlark_module;

pub mod command_output;

#[starlark_module]
pub fn eldritch_types(builder: &mut GlobalsBuilder) {
    const CommandOutput: StarlarkValueAsType<command_output::CommandOutput> = StarlarkValueAsType::new();
    fn command_output( stdout: String, stderr: String, status: i32) -> anyhow::Result<command_output::CommandOutput> {
        Ok(command_output::CommandOutput{ stdout, stderr, status })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use starlark::environment::{Module, LibraryExtension};
    use starlark::eval::Evaluator;
    use starlark::syntax::{AstModule, Dialect};
    use starlark::values::Value;

    #[test]
    fn test_command_output_type_creation() -> anyhow::Result<()>{
        let tome_filename = "test.eld";
        let tome_contents = r#"
print(type(test_command))
res = command_output(stdout='hi',stderr='',status=0)
print(res)
"#;

        let ast =  match AstModule::parse(
            tome_filename,
            tome_contents.to_string(),
            &Dialect::Extended
        ) {
            Ok(res) => res,
            Err(err) => return Err(anyhow::anyhow!("[eldritch] Unable to parse eldritch tome: {}: {} {}", err.to_string(), tome_filename.clone(), tome_contents.clone())),
        };

        let globals = GlobalsBuilder::extended_by(
            &[
                LibraryExtension::StructType,
                LibraryExtension::RecordType,
                LibraryExtension::EnumType,
                LibraryExtension::Map,
                LibraryExtension::Filter,
                LibraryExtension::Partial,
                LibraryExtension::ExperimentalRegex,
                LibraryExtension::Debug,
                LibraryExtension::Print,
                LibraryExtension::Breakpoint,
                LibraryExtension::Json,
                LibraryExtension::Abs,
                LibraryExtension::Typing,
            ]
        ).with(eldritch_types).build();
    
        let module: Module = Module::new();
        module.set(
            "test_command", 
            module.heap().alloc(command_output::CommandOutput{ 
                stdout: "Hello".to_string(), 
                stderr: "World".to_string(), 
                status: 0, 
            })
        );

        let mut eval: Evaluator = Evaluator::new(&module);

        let res: Value = match eval.eval_module(ast, &globals) {
            Ok(eval_val) => eval_val,
            Err(eval_error) => return Err(anyhow::anyhow!("[eldritch] Eldritch eval_module failed:\n{}", eval_error)),
        };

        println!("{}", res);
    
        Ok(())
    }
}