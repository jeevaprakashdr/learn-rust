struct Export<'apfle> {
    _name: &'apfle str // here the annotation is saying the struct cannot outlive beyond the reference passed to name
}

pub fn demo()  {
    
    let exporter_name = "deutsch gucken";

    let _export = Export{ _name: exporter_name };
}