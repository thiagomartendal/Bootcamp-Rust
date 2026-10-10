pub mod validadores {
    pub fn cpf(cpf: &str) -> bool {
        let cpf: Vec<u8> = cpf
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as u8)
            .collect();

        if cpf.len() != 11 || cpf.iter().all(|&d| d == cpf[0]) {
            return false;
        }

        let mut soma = 0;
        for i in 0..9 {
            soma += (cpf[i] as usize) * (10 - i);
        }
        let resto = (soma * 10) % 11;
        let primeiro_digito = if resto == 10 { 0 } else { resto as u8 };

        soma = 0;
        for i in 0..10 {
            soma += (cpf[i] as usize) * (11 - i);
        }
        let resto = (soma * 10) % 11;
        let segundo_digito = if resto == 10 { 0 } else { resto as u8 };

        primeiro_digito == cpf[9] && segundo_digito == cpf[10]
    }

    pub fn cnpj(cnpj: &str) -> bool {
        let cnpj: Vec<u8> = cnpj
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as u8)
            .collect();

        if cnpj.len() != 14 || cnpj.iter().all(|&d| d == cnpj[0]) {
            return false;
        }

        let calcular_digito = |numeros: &[u8], pesos: &[u8]| -> u8 {
            let soma: u32 = numeros
                .iter()
                .zip(pesos.iter())
                .map(|(&n, &p)| n as u32 * p as u32)
                .sum();

            let resto = soma % 11;

            if resto < 2 { 0 } else { (11 - resto) as u8 }
        };

        let d1 = calcular_digito(&cnpj[..12], &[5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]);

        if d1 != cnpj[12] {
            return false;
        }

        let d2 = calcular_digito(&cnpj[..13], &[6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]);

        d2 == cnpj[13]
    }
}
