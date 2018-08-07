package getFromDB

import java.sql.DriverManager
import java.sql.ResultSet

import com.kms.katalon.core.annotation.Keyword
import com.mysql.jdbc.Connection
import com.mysql.jdbc.Statement

public class getOneValueOnOneColumn {
	private static Connection connection = null;



	@Keyword

	def executeQuery(String queryString, String columnName) {
		String value_output = null
		Class.forName('com.ibm.db2.jcc.DB2Driver')
		if(connection != null && !connection.isClosed()){

			connection.close()
		}
		connection = DriverManager.getConnection("jdbc:db2://RDZUT01.HEB.COM:446/DB2R", "SVCT_DCM", "p9rty28j")
		Statement stm = connection.createStatement()

		ResultSet resultSet = stm.executeQuery(queryString)
		while (resultSet.next()) {
			value_output = resultSet.getObject(columnName).trim()
		}
		return value_output
	}
}
