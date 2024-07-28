use serde::{Serialize, Deserialize};
use mongodb::

#[derive(Debug,Serialize, Deserialize)]
pub struct InitialParameter{
    pub sg:String,
    pub prv:String,
    pub mt:String,
    pub bn:String,
    pub mf1_st:String,
    pub mf2_st:String,
    pub ws:String,
    pub mf_shd:String,
    pub sms:String,
    pub to:String
}

#[derive(Debug,Serialize, Deserialize)]
pub struct StatusUpdates {
    pub ut:String,
    pub sg:String,
    pub mf1_st:String,
    pub mf2_st:String,
    pub br_rn:String,
    pub pr:String,
    pub wt:String
}

#[derive(Debug,Serialize, Deserialize)]
pub struct ManifoldRefills {
    pub mf1_st:String
}

#[derive(Debug,Serialize, Deserialize)]
pub struct ManifoldShifts {
    pub mf_on:String,
    pub pr:String
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Command {
    pub to:String,
    pub cl1:String,
    pub cl2:String,
    pub stl:String,
    pub br:String,
    pub mfd:String,
    pub mpd:String,
    pub ws:String,
    pub sms:String
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Response {
    pub res:String
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Device {
    pub _id:
    pub initial_parameters:InitialParameter,
    pub status_updates:Vec<StatusUpdates>,
    pub manifold_refills:Vec<ManifoldRefills>,
    pub manifold_shifts:Vec<ManifoldShifts>,
    pub commands:Vec<Command>,
    pub responses:Vec<Response>
}