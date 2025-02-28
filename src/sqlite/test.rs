#[derive(Serialize, Deserialize, Debug)]
pub struct TestData {
    trash: String,
    something: bool,
    lies: i64,
}
impl TestData{
    pub fn new(trash: String, something: bool, lies: i64) -> TestData{
        TestData{
            trash, something, lies
        }
    }
}
pub fn create_test_tables(&self) {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS TestData(
    trash TEXT NOT NULL,
    something INT,
    lies INT
    )",(),
        );
    }
    pub fn write_test_tables(&self, data: Vec<TestData>){
        // data.iter().map(|d|{
        //     self.conn.execute("INSERT INTO TestData (trash, something, lies) VALUES (:trash, :something, :lies)", to_params_named(&d).unwrap().to_slice().as_slice()).unwrap();
        // });
        for d in data{
            self.conn.execute("INSERT INTO TestData (trash, something, lies) VALUES (:trash, :something, :lies)", to_params_named(&d).unwrap().to_slice().as_slice()).unwrap();
        }
    }
    pub fn load_test_tables(&self) -> Vec<TestData>{
        let mut statement = self.conn.prepare("SELECT * FROM TestData").unwrap();
        let data = from_rows::<TestData>(statement.query([]).unwrap());
        let mut output = data.map(|f| f.unwrap()).collect();
        output
    }
        pub fn new_test() -> Result<DBConnector> {
        Self::new("test".to_string())
    }