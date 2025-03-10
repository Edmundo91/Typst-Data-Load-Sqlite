# Typst-Data-Load-Sqlite

Uma funcionalidade para executar queries SQL em um banco SQLite diretamente no Typst.

## 🚀 Como Usar

1. Adicione a função `sqlite` ao seu projeto Typst.
2. Passe o caminho do banco de dados e uma query SQL como argumento.
3. O retorno será um array contendo os resultados.

### 📄 Exemplo

```typst
#let result = sqlite("banco.db", "SELECT * FROM users")

#for row in result {
  [Nome: #row["name"], Idade: #row["age"]]
}
```

## 📦 Requisitos

- [Typst](https://typst.app/)
- [rusqlite](https://github.com/rusqlite/rusqlite)

## 📜 Licença
MIT

