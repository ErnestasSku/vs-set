use std::collections::HashMap;


pub fn build_map() -> HashMap<String, String>
{
    let mut map = HashMap::default();

    map.insert("Godot".into(), GODOT.into());
    map.insert("Salesforce".into(), SALESFORCE.into());

    map
}

const GODOT: &str = r#"
{
    "files.exclude": {
        "**/.git": true,
        "**/.svn": true,
        "**/.hg": true,
        "**/CVS": true,
        "**/.DS_Store": true,
        "**/*.a": true,
        "**/*.o": true,
        "**/__pycache__": true,
        "**/*.obj": true,
        "bin": true,
        
        "core/config": true,
        "core/debugger": true,
        "core/error": true,
        "core/extension": true,
        "core/object": true,
        "core/string": true,
        "core/templates": true,
        "core/variant": true,
    },
    "C_Cpp.errorSquiggles": "Disabled",
    "files.associations": {
        "atomic": "cpp",
        "condition_variable": "cpp",
        "forward_list": "cpp",
        "fstream": "cpp",
        "future": "cpp",
        "memory": "cpp",
        "mutex": "cpp",
        "shared_mutex": "cpp",
        "stop_token": "cpp"
    }
    
}
"#;

const SALESFORCE: &str = r#"
{
    
    "files.exclude": {
        // "**/*.map": true
        "**/.sf" : true,
        "**/.sfdx": true,
        "**/.husky": true,
        "**/config": true,
        "sfdx-project.json": true,
        "README.md": true,
        
        //Not sure about these
        "node_modules": true,
        "jest.config.js": true,
        ".eslintignore": true,
        ".forceignore": true,
    },

    "search.exclude": {
        "**/node_modules": true,
        "**/bower_components": true,
        "**/*.code-search": true, 
        "**/package-lock.json": true,
        "**/package.json": true,
    },

    "explorer.fileNesting.enabled": true,
    "explorer.fileNesting.expand": false,
    "explorer.fileNesting.patterns": {
        "*.cls" : "${capture}.cls-meta.xml",
        "*.cmp" : "${capture}.cmp-meta.xml",
        "*.js": "${capture}.js-meta.xml",
        "*.page": "${capture}.page-meta.xml",
        "*.trigger": "${capture}.trigger-meta.xml",
        "*.component": "${capture}.component-meta.xml",
        "package.json": "package-lock.json",
        
    }
}
"#;