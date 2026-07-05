const RFC_LONGITUD_PF: usize = 13;
const RFC_LONGITUD_PM: usize = 12;
const TEL_LONGITUD: usize = 10;
const CP_LONGITUD: usize = 5;

fn validar_rfc(rfc: &str) -> Result<(), String> {
    let longitud = rfc.len();
    if longitud != RFC_LONGITUD_PF && longitud != RFC_LONGITUD_PM {
        return Err(format!("Longitud incorrecta ({} caracteres)", longitud));
    }
    let bytes = rfc.as_bytes();
    for i in 0..4 {
        if !bytes[i].is_ascii_alphabetic() {
            return Err("Los primeros 4 caracteres deben ser letras".into());
        }
    }
    for i in 4..10 {
        if i >= bytes.len() || !bytes[i].is_ascii_digit() {
            return Err(format!("El carácter en posición {} debe ser dígito", i + 1));
        }
    }
    Ok(())
}

fn validar_email(email: &str) -> Result<(), String> {
    if !email.contains('@') {
        return Err("Falta el @".into());
    }
    let partes: Vec<&str> = email.split('@').collect();
    if partes.len() != 2 {
        return Err("Sólo debe haber un @".into());
    }
    if partes[0].is_empty() {
        return Err("Falta la parte local".into());
    }
    if !partes[1].contains('.') {
        return Err("El dominio debe tener al menos un punto".into());
    }
    Ok(())
}

fn validar_telefono(tel: &str) -> Result<(), String> {
    if tel.len() != TEL_LONGITUD {
        return Err(format!("Debe tener {} dígitos (tiene {})", TEL_LONGITUD, tel.len()));
    }
    if !tel.chars().all(|c| c.is_ascii_digit()) {
        return Err("Sólo dígitos".into());
    }
    Ok(())
}

fn validar_cp(cp: &str) -> Result<(), String> {
    if cp.len() != CP_LONGITUD {
        return Err(format!("Debe tener {} dígitos", CP_LONGITUD));
    }
    if !cp.chars().all(|c| c.is_ascii_digit()) {
        return Err("Sólo dígitos".into());
    }
    Ok(())
}

fn main() {
    let datos = vec![
        ("XAXX010101000", "contacto@empresa.com", "5555123456", "01234", "Constructora S.A."),
        ("XEXX010101000", "mal email",            "5551234",    "1234",  ""),
        ("XAXX010101AB3", "info@empresa.com.mx",  "5512345678", "11550", "Distribuidora del Norte"),
    ];

    println!("Validador de clientes - ERP/CRM\n");
    for (rfc, email, tel, cp, razon) in &datos {
        println!("Cliente: {}", razon);
        println!("  RFC:      {} {:?}", rfc, validar_rfc(rfc));
        println!("  Email:    {} {:?}", email, validar_email(email));
        println!("  Teléfono: {} {:?}", tel,   validar_telefono(tel));
        println!("  CP:       {} {:?}", cp,    validar_cp(cp));
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc_valido() {
        assert!(validar_rfc("XAXX010101000").is_ok());
    }

    #[test]
    fn rfc_muy_corto() {
        assert!(validar_rfc("XAXX").is_err());
    }

    #[test]
    fn rfc_letras_iniciales_invalidas() {
        assert!(validar_rfc("1234010101000").is_err());
    }

    #[test]
    fn email_valido() {
        assert!(validar_email("a@b.co").is_ok());
    }

    #[test]
    fn email_sin_arroba() {
        assert!(validar_email("ab.co").is_err());
    }

    #[test]
    fn email_dos_arrobas() {
        assert!(validar_email("a@b@c.co").is_err());
    }

    #[test]
    fn telefono_valido() {
        assert!(validar_telefono("5512345678").is_ok());
    }

    #[test]
    fn telefono_muy_corto() {
        assert!(validar_telefono("1234").is_err());
    }

    #[test]
    fn cp_valido() {
        assert!(validar_cp("11550").is_ok());
    }

    #[test]
    fn cp_muy_corto() {
        assert!(validar_cp("1234").is_err());
    }
}
