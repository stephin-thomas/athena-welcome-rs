use csv::ReaderBuilder;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
#[derive(Debug, serde::Deserialize, Clone)]
pub struct Record {
    pub role: String,
    pub tool: String,
    pub desc: String,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct ToolRecipe {
    pub tool: String,
    pub desc: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct HackingVariables {
    pub variable: String,
    pub path: String,
    pub category: String,
}

pub fn read_csv_data<T>(path: impl AsRef<Path>) -> Vec<T>
where
    T: DeserializeOwned,
    T: std::fmt::Debug,
{
    let mut csv_reader: csv::Reader<File> = ReaderBuilder::new()
        .delimiter(b';')
        .from_path(path)
        .expect("Error reading csv file");
    let rec_iter = csv_reader.deserialize();
    let records: Vec<T> = rec_iter
        .filter(|rec| rec.is_ok())
        .map(|rec| rec.unwrap())
        .collect();
    // println!("Records {:?}", records);
    return records;
}
impl ToTokens for Record {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let role = self.role.clone();
        let tool = self.tool.clone();
        let desc = self.desc.clone();
        let tk_stream = quote! {
            Record{
                role: String::from(#role),
                tool: String::from(#tool),
                desc: String::from(#desc),
            }
        };
        tokens.extend(tk_stream);
    }
}
impl ToTokens for HackingVariables {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path = self.path.clone();
        let variable = self.variable.clone();
        let category = self.category.clone();
        let tk_stream = quote! {
            HackingVariables{
                path: String::from(#path),
                variable: String::from(#variable),
                category: String::from(#category),
            }
        };
        tokens.extend(tk_stream);
    }
}

impl ToTokens for ToolRecipe {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tool = self.tool.clone();
        let desc = self.desc.clone();
        let tk_stream = quote! {
            ToolRecipe{
                tool: String::from(#tool),
                desc: String::from(#desc),
            }
        };
        tokens.extend(tk_stream);
    }
}
pub fn main() {
    glib_build_tools::compile_resources(
        &["src/gui/assets"],
        "src/gui/img_resource.xml",
        "embed_assets.gresource",
    );

    println!("cargo:rerun-if-changed=assets");
    let out_dir = PathBuf::from("src"); // Replace with your output directory
    let final_file = out_dir.join("csv_data.rs");
    println!("cargo:rerun-if-changed={:?}", final_file);
    // Define the path to your CSV file
    let role_csv_path = PathBuf::from("csv/roles.csv");
    let hacking_csv_path = PathBuf::from("csv/hacking_variables.csv");
    let tools_recipe_csv_path = PathBuf::from("csv/tools_recipe.csv");
    let record_from_csv = read_csv_data::<Record>(role_csv_path);
    let tools_recipe_csv = read_csv_data::<ToolRecipe>(tools_recipe_csv_path);
    let hacking_variables_from_csv = read_csv_data::<HackingVariables>(hacking_csv_path);
    let mut role_tools_tk_stream = TokenStream::new();
    let mut hacking_vars_tk_stream = TokenStream::new();
    let mut tools_recipe_tk_stream = TokenStream::new();
    for record in record_from_csv.into_iter() {
        let strt = quote!(#record,);
        role_tools_tk_stream.extend(strt);
    }
    for hacking_var in hacking_variables_from_csv.into_iter() {
        let strt = quote!(#hacking_var,);
        hacking_vars_tk_stream.extend(strt);
    }
    for tools_recipe in tools_recipe_csv.into_iter() {
        let strt = quote!(#tools_recipe,);
        tools_recipe_tk_stream.extend(strt);
    }
    let final_ts = quote! {
    //Auto generated file from build script do not modify
       use crate::utils::Record;
       use crate::utils::HackingVariables;
       use crate::utils::ToolRecipe;
    pub fn get_roles()->Vec<Record>
       {
       let roles: Vec<Record> = vec![#role_tools_tk_stream];
       return roles;}
    pub fn get_hk_vars()->Vec<HackingVariables>
       {
        let hacking_vars: Vec<HackingVariables> = vec![#hacking_vars_tk_stream];
        return hacking_vars;}
    pub fn get_tools_recipe()->Vec<ToolRecipe>
       {
        let tools_recipe: Vec<ToolRecipe> = vec![#tools_recipe_tk_stream];
        return tools_recipe;}
    };

    // println!("{}", final_ts.to_string());
    let syntax_tree =
        syn::parse_file(final_ts.to_string().as_str()).expect("Error parsing syntax tree");
    let formatted_syntax = prettyplease::unparse(&syntax_tree);
    let mut file = File::create(final_file).expect("Failed to create file");
    write!(file, "{}", formatted_syntax).expect("Failed to write code");
}
