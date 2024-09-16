use serde_json::Value;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("What is your name? : ");

    let json_str = r#"
    {
        "rows": [
            {
                "ProtonAccount": "tommccann",
                "ICPrincipal": "gpurw-f4h72-qwdnm-vmexj-xnhww-us2kt-kbiua-o3y4u-bzduw-qhb7a-jqe",
                "Amount": 100,
                "DateTime": 1725805695
            },
            {
                "ProtonAccount": "judetan",
                "ICPrincipal": "22gak-zasla-2cj5r-ix2ds-4kaxw-lrgtq-4zjul-mblvf-gkhsi-fzu3j-cae",
                "Amount": 40,
                "DateTime": 1725805791
            }
        ]
    }
    "#;

    // Parse the JSON string
    let parsed: Value = serde_json::from_str(json_str)?;

    // Access the second record in the array and retrieve the Amount
    if let Some(amount) = parsed["rows"][1]["Amount"].as_i64() {
        println!("Amount from the second record: {}", amount);
    } else {
        println!("Failed to retrieve Amount from the second record");
    }

    Ok(())
}
