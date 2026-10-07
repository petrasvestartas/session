        let mut phases = Vec::new();
        let result: Result<Vec<Vec<u8>>, String> = async {
            let mut values = Vec::new();
            for url in request.urls {
                let started = crate::browser_phase::now();
                let value = crate::source_fetch::fetch(&url, &signal).await;
                phases.extend(crate::browser_network::measured(&url, started, value.is_ok(),
                    value.as_ref().map_or(0, |bytes| bytes.len() as u64)));
                values.push(value?);
            }
            Ok(values)
        }.await;
        let Some((keys, intent)) = shared.borrow_mut().finish(request.ticket, result.is_err()) else { return; };
        for phase in phases { let _ = crate::browser_report::phase(phase); }