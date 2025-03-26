# Conteúdo de `sqlite.rs`

Em typstlibrary/src/loading, adicione o arquivo sqlite.rs

#[func(scope, title = "SQLite")]
pub fn sqlite(
    engine: &mut Engine,
    source: Spanned<DataSource>,
    query: String,
) -> SourceResult<Value> {
    let db_path = match &source.v {
        DataSource::Path(path) => path.to_string(),
        DataSource::Bytes(_) => {
            return Err(EcoVec::from(vec![SourceDiagnostic {
                severity: Severity::Error,
                span: Span::from_raw(NonZeroU64::new(1).unwrap()),
                message: "Invalid data: expected a file path, not raw bytes.".into(),
                trace: EcoVec::new(),
                hints: EcoVec::new(),
            }]));
        }
    };

    let conn = Connection::open(db_path).map_err(|err| {
        EcoVec::from(vec![SourceDiagnostic {
            severity: Severity::Error,
            span: Span::from_raw(NonZeroU64::new(0).unwrap()),
            message: format!("Database error: {}", err).into(),
            trace: EcoVec::new(),
            hints: EcoVec::new(),
        }])
    })?;

    let mut stmt = conn.prepare(&query).map_err(|err| {
        EcoVec::from(vec![SourceDiagnostic {
            severity: Severity::Error,
            span: Span::from_raw(NonZeroU64::new(0).unwrap()),
            message: format!("SQL error: {}", err).into(),
            trace: EcoVec::new(),
            hints: EcoVec::new(),
        }])
    })?;

    let column_count = stmt.column_count();
    let column_names: Vec<String> = stmt.column_names().into_iter().map(|s| s.to_string()).collect();

    let rows = stmt
        .query_map([], |row| {
            let mut dict = Dict::new();
            for i in 0..column_count {
                let column_name = column_names[i].clone();
                let value = match row.get_ref_unwrap(i) {
                    rusqlite::types::ValueRef::Integer(i) => Value::Int(i),
                    rusqlite::types::ValueRef::Real(f) => Value::Float(f),
                    rusqlite::types::ValueRef::Text(s) => Value::Str(Str::from(String::from_utf8_lossy(s).to_string())),
                    _ => Value::None,
                };
                dict.insert(Str::from(column_name), value);
            }
            Ok(Value::Dict(dict))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Value::Array(rows.into_iter().collect::<Array>()))
} 








#[scope]
impl sqlite {
    
    ///
    /// This function is deprecated. 
    /// directly.
    #[func(title = "Decode JSON")]
    pub fn decode(
        engine: &mut Engine,
        /// JSON data.
        data: Spanned<Readable>,
    ) -> SourceResult<Value> {
        json(engine, data.map(Readable::into_source))
    }
}
