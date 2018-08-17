import java.text.SimpleDateFormat as SimpleDateFormat

import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

String date = '2018-08-14 21:22:36.437046'

input_dateTime_Format = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss");
input_dateTime_Parse = input_dateTime_Format.parse(date);
simpleFormat_dt = new SimpleDateFormat("MM/dd/yyyy @ HH:mm:ss");
out_dateTime_Parse = simpleFormat_dt.format(input_dateTime_Parse).toString()

WebUI.verifyEqual(out_dateTime_Parse, '08/14/2018 @ 21:22:36')

date_format = new date.format("MM/dd/yyyy @ HH:mm:ss");

WebUI.verifyEqual(date_format, '08/14/2018 @ 21:22:36')