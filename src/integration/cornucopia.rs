// This file was generated with `cornucopia`. Do not modify.

pub mod types {
    pub mod public {
        #[derive(Debug, postgres_types::ToSql, postgres_types::FromSql)]
        #[postgres(name = "custom_composite")]
        #[derive(Clone)]
        pub struct CustomComposite {
            pub such_cool: i32,
            pub wow: String,
            pub nice: super::public::SpongebobCharacter,
        }

        #[derive(Debug, postgres_types::ToSql, postgres_types::FromSql)]
        #[postgres(name = "spongebob_character")]
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub enum SpongebobCharacter {
            Bob,
            Patrick,
            Squidward,
        }
    }
}

pub mod queries {
    pub mod module_1 {
        pub async fn insert_book_one<T: cornucopia_client::GenericClient>(
            client: &T,
        ) -> Result<(), tokio_postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO Book (title)
VALUES ('bob');
",
                )
                .await?;
            let _ = client.execute(&stmt, &[]).await?;
            Ok(())
        }

        pub async fn insert_book_zero_or_one<T: cornucopia_client::GenericClient>(
            client: &T,
        ) -> Result<(), tokio_postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO Book (title)
VALUES ('alice');
",
                )
                .await?;
            let _ = client.execute(&stmt, &[]).await?;
            Ok(())
        }

        pub async fn insert_book_zero_or_more<T: cornucopia_client::GenericClient>(
            client: &T,
        ) -> Result<(), tokio_postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO Book (title)
VALUES ('carl');
",
                )
                .await?;
            let _ = client.execute(&stmt, &[]).await?;
            Ok(())
        }

        pub async fn insert_stream<T: cornucopia_client::GenericClient>(
            client: &T,
        ) -> Result<(), tokio_postgres::Error> {
            let stmt = client
                .prepare(
                    "INSERT INTO Book (title)
VALUES ('dominic');
",
                )
                .await?;
            let _ = client.execute(&stmt, &[]).await?;
            Ok(())
        }
    }

    pub mod module_2 {
        pub async fn authors<T: cornucopia_client::GenericClient>(
            client: &T,
        ) -> Result<Vec<(i32, String, String)>, tokio_postgres::Error> {
            use futures::{StreamExt, TryStreamExt};
            let stmt = client
                .prepare(
                    "SELECT
*
FROM
Author;
",
                )
                .await?;
            let res = client
                .query_raw(&stmt, std::iter::empty::<i32>())
                .await?
                .map(|res| {
                    res.map(|res| {
                        let return_value_0: i32 = res.get(0);
                        let return_value_1: String = res.get(1);
                        let return_value_2: String = res.get(2);
                        (return_value_0, return_value_1, return_value_2)
                    })
                })
                .try_collect()
                .await?;
            Ok(res)
        }

        pub async fn authors_stream<T: cornucopia_client::GenericClient>(
            client: &T,
        ) -> Result<
            impl futures::Stream<Item = Result<(i32, String, String), tokio_postgres::Error>>,
            tokio_postgres::Error,
        > {
            use futures::{StreamExt, TryStreamExt};
            let stmt = client
                .prepare(
                    "SELECT
*
FROM
Author;
",
                )
                .await?;
            let row_stream = client
                .query_raw(&stmt, std::iter::empty::<i32>())
                .await?
                .map(|res| {
                    res.map(|res| {
                        let return_value_0: i32 = res.get(0);
                        let return_value_1: String = res.get(1);
                        let return_value_2: String = res.get(2);
                        (return_value_0, return_value_1, return_value_2)
                    })
                });
            Ok(row_stream.into_stream())
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Books {
            pub title: String,
        }
        pub async fn books<T: cornucopia_client::GenericClient>(
            client: &T,
        ) -> Result<Vec<super::super::queries::module_2::Books>, tokio_postgres::Error> {
            use futures::{StreamExt, TryStreamExt};
            let stmt = client
                .prepare(
                    "SELECT
Title
FROM
Book;
",
                )
                .await?;
            let res = client
                .query_raw(&stmt, std::iter::empty::<i32>())
                .await?
                .map(|res| {
                    res.map(|res| {
                        let return_value_0: String = res.get(0);
                        super::super::queries::module_2::Books {
                            title: return_value_0,
                        }
                    })
                })
                .try_collect()
                .await?;
            Ok(res)
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BooksOptRetParam {
            pub title: Option<String>,
        }
        pub async fn books_opt_ret_param<T: cornucopia_client::GenericClient>(
            client: &T,
        ) -> Result<Vec<super::super::queries::module_2::BooksOptRetParam>, tokio_postgres::Error>
        {
            use futures::{StreamExt, TryStreamExt};
            let stmt = client
                .prepare(
                    "SELECT
Title
FROM
Book;
",
                )
                .await?;
            let res = client
                .query_raw(&stmt, std::iter::empty::<i32>())
                .await?
                .map(|res| {
                    res.map(|res| {
                        let return_value_0: Option<String> = res.get(0);
                        super::super::queries::module_2::BooksOptRetParam {
                            title: return_value_0,
                        }
                    })
                })
                .try_collect()
                .await?;
            Ok(res)
        }

        pub async fn books_from_author_id<T: cornucopia_client::GenericClient>(
            client: &T,
            id: &i32,
        ) -> Result<Vec<String>, tokio_postgres::Error> {
            use futures::{StreamExt, TryStreamExt};
            let stmt = client
                .prepare(
                    "SELECT
Book.Title
FROM
BookAuthor
INNER JOIN Author ON Author.Id = BookAuthor.AuthorId
INNER JOIN Book ON Book.Id = BookAuthor.BookId
WHERE
Author.Id = $1;
",
                )
                .await?;
            let res = client
                .query_raw(&stmt, &[&id])
                .await?
                .map(|res| {
                    res.map(|row| {
                        let value: String = row.get(0);
                        value
                    })
                })
                .try_collect()
                .await?;
            Ok(res)
        }

        pub async fn author_name_by_id_opt<T: cornucopia_client::GenericClient>(
            client: &T,
            id: &i32,
        ) -> Result<Option<String>, tokio_postgres::Error> {
            let stmt = client
                .prepare(
                    "SELECT
Author.Name
FROM
Author
WHERE
Author.Id = $1;
",
                )
                .await?;
            let res = client.query_opt(&stmt, &[&id]).await?;
            let return_value = res.map(|row| {
                let value: String = row.get(0);
                value
            });
            Ok(return_value)
        }

        pub async fn author_name_by_id<T: cornucopia_client::GenericClient>(
            client: &T,
            id: &i32,
        ) -> Result<String, tokio_postgres::Error> {
            let stmt = client
                .prepare(
                    "SELECT
Author.Name
FROM
Author
WHERE
Author.Id = $1;
",
                )
                .await?;
            let res = client.query_one(&stmt, &[&id]).await?;
            let return_value: String = res.get(0);
            Ok(return_value)
        }

        pub async fn author_name_starting_with<T: cornucopia_client::GenericClient>(
            client: &T,
            s: &str,
        ) -> Result<Vec<(i32, String, i32, String)>, tokio_postgres::Error> {
            use futures::{StreamExt, TryStreamExt};
            let stmt = client
                .prepare(
                    "SELECT
BookAuthor.AuthorId,
Author.Name,
BookAuthor.BookId,
Book.Title
FROM
BookAuthor
INNER JOIN Author ON Author.id = BookAuthor.AuthorId
INNER JOIN Book ON Book.Id = BookAuthor.BookId
WHERE
Author.Name LIKE CONCAT($1::text, '%');
",
                )
                .await?;
            let res = client
                .query_raw(&stmt, &[&s])
                .await?
                .map(|res| {
                    res.map(|res| {
                        let return_value_0: i32 = res.get(0);
                        let return_value_1: String = res.get(1);
                        let return_value_2: i32 = res.get(2);
                        let return_value_3: String = res.get(3);
                        (
                            return_value_0,
                            return_value_1,
                            return_value_2,
                            return_value_3,
                        )
                    })
                })
                .try_collect()
                .await?;
            Ok(res)
        }

        pub async fn return_custom_type<T: cornucopia_client::GenericClient>(
            client: &T,
        ) -> Result<super::super::types::public::CustomComposite, tokio_postgres::Error> {
            let stmt = client
                .prepare(
                    "SELECT
col1
FROM
CustomTable;
",
                )
                .await?;
            let res = client.query_one(&stmt, &[]).await?;
            let return_value: super::super::types::public::CustomComposite = res.get(0);
            Ok(return_value)
        }

        pub async fn select_where_custom_type<T: cornucopia_client::GenericClient>(
            client: &T,
            spongebob_character: &super::super::types::public::SpongebobCharacter,
        ) -> Result<super::super::types::public::SpongebobCharacter, tokio_postgres::Error>
        {
            let stmt = client
                .prepare(
                    "SELECT
col2
FROM
CustomTable
WHERE (col1).nice = $1;
",
                )
                .await?;
            let res = client.query_one(&stmt, &[&spongebob_character]).await?;
            let return_value: super::super::types::public::SpongebobCharacter = res.get(0);
            Ok(return_value)
        }

        pub async fn select_everything<T: cornucopia_client::GenericClient>(
            client: &T,
        ) -> Result<
            (
                Vec<bool>,
                Vec<super::super::types::public::SpongebobCharacter>,
                bool,
                bool,
                i8,
                i16,
                i16,
                i16,
                i16,
                i32,
                i32,
                i32,
                i32,
                i64,
                i64,
                i64,
                i64,
                f32,
                f32,
                f64,
                f64,
                String,
                String,
                Vec<u8>,
                time::PrimitiveDateTime,
                time::PrimitiveDateTime,
                time::OffsetDateTime,
                time::OffsetDateTime,
                time::Date,
                time::Time,
                serde_json::Value,
                serde_json::Value,
                uuid::Uuid,
                std::net::IpAddr,
                eui48::MacAddress,
            ),
            tokio_postgres::Error,
        > {
            let stmt = client
                .prepare(
                    "SELECT
*
FROM
Everything;
",
                )
                .await?;
            let res = client.query_one(&stmt, &[]).await?;
            let return_value = {
                let return_value_0: Vec<bool> = res.get(0);
                let return_value_1: Vec<super::super::types::public::SpongebobCharacter> =
                    res.get(1);
                let return_value_2: bool = res.get(2);
                let return_value_3: bool = res.get(3);
                let return_value_4: i8 = res.get(4);
                let return_value_5: i16 = res.get(5);
                let return_value_6: i16 = res.get(6);
                let return_value_7: i16 = res.get(7);
                let return_value_8: i16 = res.get(8);
                let return_value_9: i32 = res.get(9);
                let return_value_10: i32 = res.get(10);
                let return_value_11: i32 = res.get(11);
                let return_value_12: i32 = res.get(12);
                let return_value_13: i64 = res.get(13);
                let return_value_14: i64 = res.get(14);
                let return_value_15: i64 = res.get(15);
                let return_value_16: i64 = res.get(16);
                let return_value_17: f32 = res.get(17);
                let return_value_18: f32 = res.get(18);
                let return_value_19: f64 = res.get(19);
                let return_value_20: f64 = res.get(20);
                let return_value_21: String = res.get(21);
                let return_value_22: String = res.get(22);
                let return_value_23: Vec<u8> = res.get(23);
                let return_value_24: time::PrimitiveDateTime = res.get(24);
                let return_value_25: time::PrimitiveDateTime = res.get(25);
                let return_value_26: time::OffsetDateTime = res.get(26);
                let return_value_27: time::OffsetDateTime = res.get(27);
                let return_value_28: time::Date = res.get(28);
                let return_value_29: time::Time = res.get(29);
                let return_value_30: serde_json::Value = res.get(30);
                let return_value_31: serde_json::Value = res.get(31);
                let return_value_32: uuid::Uuid = res.get(32);
                let return_value_33: std::net::IpAddr = res.get(33);
                let return_value_34: eui48::MacAddress = res.get(34);
                (
                    return_value_0,
                    return_value_1,
                    return_value_2,
                    return_value_3,
                    return_value_4,
                    return_value_5,
                    return_value_6,
                    return_value_7,
                    return_value_8,
                    return_value_9,
                    return_value_10,
                    return_value_11,
                    return_value_12,
                    return_value_13,
                    return_value_14,
                    return_value_15,
                    return_value_16,
                    return_value_17,
                    return_value_18,
                    return_value_19,
                    return_value_20,
                    return_value_21,
                    return_value_22,
                    return_value_23,
                    return_value_24,
                    return_value_25,
                    return_value_26,
                    return_value_27,
                    return_value_28,
                    return_value_29,
                    return_value_30,
                    return_value_31,
                    return_value_32,
                    return_value_33,
                    return_value_34,
                )
            };
            Ok(return_value)
        }
    }
}