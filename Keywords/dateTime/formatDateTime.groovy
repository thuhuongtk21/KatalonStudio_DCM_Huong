package dateTime

import java.text.SimpleDateFormat
import com.kms.katalon.core.annotation.Keyword

public class formatDateTime {

	@Keyword

	def formatDate_Time(String date, String dateType) throws Exception {
		String input_dateTime_Parse = null;
		Date out_dateTime_Parse = null;
		SimpleDateFormat input_dateTime_Format = null;
		SimpleDateFormat simpleFormat_dt = null;
		switch (dateType) {
			case "@ yyyy-MM-dd HH:mm:ss":
				input_dateTime_Format = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss");
				input_dateTime_Parse = input_dateTime_Format.parse(date);
				simpleFormat_dt = new SimpleDateFormat("MM/dd/yyyy @ HH:mm:ss");
				out_dateTime_Parse = simpleFormat_dt.format(input_dateTime_Parse).toString()

				break;
			case "yyyy-MM-dd HH:mm:ss":
				input_dateTime_Format = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss");
				input_dateTime_Parse = input_dateTime_Format.parse(date);
				simpleFormat_dt = new SimpleDateFormat("MM/dd/yyyy HH:mm:ss");
				out_dateTime_Parse = simpleFormat_dt.format(input_dateTime_Parse)
				break;
			case "yyyy-mm-dd":
				input_dateTime_Format = new SimpleDateFormat("yyyy-mm-dd");
				input_dateTime_Parse = input_dateTime_Format.parse(date);
				simpleFormat_dt = new SimpleDateFormat("mm-dd-yyyy");
				out_dateTime_Parse = simpleFormat_dt.format(input_dateTime_Parse)
				break;
			case "/yyyy-mm-dd":
				input_dateTime_Format = new SimpleDateFormat("yyyy-mm-dd");
				input_dateTime_Parse = input_dateTime_Format.parse(date);
				simpleFormat_dt = new SimpleDateFormat("mm/dd/yyyy");
				out_dateTime_Parse = simpleFormat_dt.format(input_dateTime_Parse)
				break;
		}
	}
}
