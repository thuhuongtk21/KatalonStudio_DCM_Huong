package dateTime

import java.text.SimpleDateFormat
import com.kms.katalon.core.annotation.Keyword
import java.text.DateFormat

public class formatDateTime {

	@Keyword

	def formatDate_Time(String date, String dateType) throws Exception {
		Date input_dateTime_Parse = null;
		  SimpleDateFormat input_dateTime_Format = null;
		  SimpleDateFormat simpleFormat_dt = null;
		switch (dateType) {
			case "yyyy-MM-dd HH:mm:ss to MM/dd/yyyy @ HH:mm:ss":
				input_dateTime_Format = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss");
				input_dateTime_Parse = input_dateTime_Format.parse(date);
				simpleFormat_dt = new SimpleDateFormat("MM/dd/yyyy @ HH:mm:ss");
				break;
			case "yyyy-MM-dd HH:mm:ss to MM/dd/yyyy HH:mm:ss":
				input_dateTime_Format = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss");
				input_dateTime_Parse = input_dateTime_Format.parse(date);
				simpleFormat_dt = new SimpleDateFormat("MM/dd/yyyy HH:mm:ss");				
				break;
			case "yyyy-mm-dd to mm-dd-yyyy":
				input_dateTime_Format = new SimpleDateFormat("yyyy-mm-dd");
				input_dateTime_Parse = input_dateTime_Format.parse(date);
				simpleFormat_dt = new SimpleDateFormat("mm-dd-yyyy");				
				break;
			case "/yyyy-mm-dd to mm/dd/yyyy":
				input_dateTime_Format = new SimpleDateFormat("yyyy-mm-dd");
				input_dateTime_Parse = input_dateTime_Format.parse(date);
				simpleFormat_dt = new SimpleDateFormat("mm/dd/yyyy");				
				break;
		}		
		return simpleFormat_dt.format(input_dateTime_Parse).toString();
		

	}
}
