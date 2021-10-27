pub struct Library {}

impl ftd::p2::Library for Library {
    fn get(&self, name: &str) -> Option<String> {
        // To update the wget structure, update this.
        let wget_ftd = indoc::indoc!("
            -- var verbose: false

            -- record config:
            auto_resume: boolean
            user_agent: string


            -- or-type pass_or_pat:

            --- password:
            password: caption

            --- pat:
            pat: caption

            -- record proxy:
            server: string
            port: string
            username: string
            auth: list pass_or_pat

            -- record header:
            key: string
            value: string

            -- list headers:
            type: header
        ").to_string();
        if name == "wget" {
            return Some(wget_ftd);
        }
        else {
            unimplemented!("nothing else is supported");
        }
    }
    fn process(&self, section: &ftd::p1::Section, doc:  &ftd::p2::TDoc) -> ftd::p1::Result<ftd::Value> {
        // This is unimplemented for now!
        unimplemented!("process not implemented!")
    }
}