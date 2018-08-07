package connectToDB

import java.sql.*

import com.kms.katalon.core.annotation.Keyword


public class connectToDB2Test_Variable {
	private static Connection connection = null;

	@Keyword

	def connectToBD2Test_Testing(String url, String user, String password){
		Class.forName('com.ibm.db2.jcc.DB2Driver')
		connection = DriverManager.getConnection(url, user, password)
		return connection
	}

	@Keyword

	def executeQuery(String queryString, String url, String user, String password) {
		Connection connection = connectToBD2Test_Testing(url, user, password)

		Statement stm = connection.createStatement()

		ResultSet rs = stm.executeQuery(queryString)

		return rs
	}

	@Keyword

	def closeDatabaseConnection() {

		if(connection != null && !connection.isClosed()){

			connection.close()
		}

		connection = null
	}

	@Keyword

	def execute(String queryString) {

		Statement stm = connection.createStatement()

		boolean result = stm.execute(queryString)

		return result
	}
}
