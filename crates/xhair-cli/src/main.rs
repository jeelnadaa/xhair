fn main() -> anyhow::Result<()> {
    println!("xhair-cli v{}", env!("CARGO_PKG_VERSION"));
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cli_version() {
        assert!(!env!("CARGO_PKG_VERSION").is_empty());
    }
}
