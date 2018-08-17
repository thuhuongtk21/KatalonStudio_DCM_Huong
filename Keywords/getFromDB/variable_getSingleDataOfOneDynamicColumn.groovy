package getFromDB

import java.sql.*
import java.sql.Statement;
import java.lang.String

import com.kms.katalon.core.annotation.Keyword


public class variable_getSingleDataOfOneDynamicColumn {
	private static Connection connection = null;



	@Keyword

	def executeQuery(String queryString, String schema, String columnName) {
		String query_execute = String.format(queryString, schema)

		Class.forName('com.ibm.db2.jcc.DB2Driver')
		if(connection != null && !connection.isClosed()){

			connection.close()
		}
		connection = DriverManager.getConnection("jdbc:db2://RDZUT01.HEB.COM:446/DB2R", "SVCT_DCM", "p9rty28j")
		Statement stm = connection.createStatement()
		String output = null


		ResultSet resultSet = stm.executeQuery(query_execute)
		while (resultSet.next()) {
			output = resultSet.getObject(columnName).trim()
		}
		return output
	}

	@Keyword

	def closeDatabaseConnection() {

		if(connection != null && !connection.isClosed()){

			connection.close()
		}

		connection = null
	}
}
