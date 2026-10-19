 pub fn prompt(&self)->String{if let Some(i)=self.asking{format!("{} <{}>: type a number · Enter keeps it",self.params[i].name,self.params[i].value)}else{format!("{} · pick original sources ({} picked) · Enter creates · Esc cancels",self.kind.name(),self.picked.len())}}
 pub fn buttons(&self)->Vec<(String,String)>{self.params.iter().map(|p|(format!("{}={}",p.name,p.value),p.name.into())).chain([("Create".into(),"".into()),("Cancel".into(),"Escape".into())]).collect()}
    pub fn take(&mut self, word: &str) -> Result<(), String> {
        let (name, number) = match word.split_once('=') {
            Some((name, number)) => (Some(name), Some(number)),
            None if word.parse::<f64>().ok().filter(|v|v.is_finite()).is_some() => (None, Some(word)),
            None => (Some(word), None),
        };

        if let Some(name) = name {
            let at = self
                .params
                .iter()
                .position(|param| param.name.eq_ignore_ascii_case(name))
                .ok_or_else(|| {
                    format!(
                        "`{word}` is not a value of {} · {}",
                        self.kind.name(),
                        self.values()
                    )
                })?;
            self.asking = Some(at);
        }

        let Some(number) = number else {
            return Ok(());
        };
        let value = number.parse::<f64>().ok().filter(|v|v.is_finite())
            .filter(|value| *value > 0.0)
            .ok_or_else(|| format!("`{number}` is not a positive length"))?;

        if self.params.is_empty() {
            return Err(format!(
                "{} takes no values · pick {}",
                self.kind.name(), "fitting original sources"
            ));
        }

        let at = self.asking.take().unwrap_or(self.next % self.params.len());
        self.params[at].value = value;
        self.next = at + 1;
        Ok(())
    }
}
