package getFromDB

import java.sql.*
import java.sql.Statement;

import com.kms.katalon.core.annotation.Keyword
import java.lang.String

public class getSingleDataOfOneDynamicColumn_SwitchCase {
	private static Connection connection = null;



	@Keyword

	def executeQuery(String queryString, String columnName) {
		Class.forName('com.ibm.db2.jcc.DB2Driver')
		if(connection != null && !connection.isClosed()){

			connection.close()
		}
		connection = DriverManager.getConnection("jdbc:db2://RDZUT01.HEB.COM:446/DB2R", "SVCT_DCM", "p9rty28j")
		Statement stm = connection.createStatement()
		String output = null


		ResultSet resultSet = stm.executeQuery(queryString)
		while (resultSet.next()) {
			output = resultSet.getObject(columnName).trim()
			String output_String = output.toString()
			switch(output){
				case "s303174":
					output= output.replace('s303174', 'S303174 Schuehle,Celeste')
					break
			}			
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
