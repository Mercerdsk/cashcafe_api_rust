use crate::repository::database_connection::db_connection;
use crate::models::request_models::output_json;


pub async fn player_creation_sp()->Result<Vec<output_json>,Box<dyn std::error::Error>>{
    let client = db_connection().await?;
    let mut array_data:Vec<output_json> = Vec::new();
    let res = client.query("SELECT  id ,name ,official_name  from RoanuzFantasy.dbo.countries;",&[]).await?;
            match res {
                Ok(res_stream)=>{
                    let res_value=res_stream.into_results().await;
                    match res_value{
                        Ok(table_vec)=>{
                            for table_data in &table_vec[0]{
                                let id:i32 = table_data.get("id").unwrap_or(0);
                                let name:&str = table_data.get("name").unwrap_or("null");
                                let full_name:&str = table_data.get("official_name").unwrap_or("null");
                                let out_json = output_json{
                                    id:id,
                                    name:String::from(name),
                                    official_name:String::from(full_name),
                                };
                                array_data.push(out_json);
                            }
                            return Ok(array_data);
                        }
                        Err(e)=>{return None;}
                    }
                }
                Err(e)=>{return None;}
            }
        }
    