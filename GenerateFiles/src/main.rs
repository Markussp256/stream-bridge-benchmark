
use std::fs::write;

fn main() -> std::io::Result<()> {
    let path_orig="..\\".to_string();
    let path=||path_orig.clone();
    for i in 1..=2 {
        let class_name="Class".to_string() + &i.to_string();
        let class_file_name="class".to_string() + &i.to_string();
        {
            let header_txt="#include <iosfwd>\n\n".to_string() +
            "class " + &class_name + "\n"+
            "{\n"+
                "\tvoid print(std::ostream&) const;\n" +
            "};";
            write(path()+"BaseLine\\Classes\\" + &class_file_name + ".h",header_txt.clone())?;
            write(path()+"WithPCH\\Classes\\" + &class_file_name + ".h",header_txt)?;
        }
        {
            let header_txt="#include \"iosfwd-bridge.h\"\n\n".to_string() +
            "class " + &class_name + "\n"+
            "{\n"+
                "\tvoid print(stream::ostream&) const;\n" +
            "};";
            write(path()+"WithStreamBridge\\Classes\\" + &class_file_name + ".h",header_txt)?;
        }
        let src_txt=|namespace:&str,include:&str|
        {
            "#include \"".to_string() + &class_file_name + ".h\"\n#include " + include + "\n\n" +
            "void " + &class_name  + " :: print(" + namespace + "::ostream& os) const\n"+
            "{\n"+
                "\tos << \"this is " + &class_name + "\";\n" +
            "};"
        };
        write(path()+"BaseLine\\Classes\\" + &class_file_name + ".cpp",src_txt("std","<ostream>"))?;
        write(path()+"WithPCH\\Classes\\" + &class_file_name + ".cpp",src_txt("std","\"precompiled.h\""))?;
        write(path()+"WithStreamBridge\\Classes\\" + &class_file_name + ".cpp",src_txt("stream","\"ostream-bridge.h\""))?;
    }
    Ok(())
}
