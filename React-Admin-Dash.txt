# React Admin Dashboard / Ticket System

	in VSC add prettier - code formatter
	* Extesion section

	```
	npx create-react-app [name of app]
	```
	* this will automatically create the boiler plate template needed
	* cd [newly created folder]

	```
	npm i @mui/material @emotion/react @emotion/styled @mui/x-data-grid @mui/icons-material react-router-dom@6 
	react-pro-sidebar formik yup @fullcalendar/core @fullcalendar/daygrid @fullcalendar/timegrid @fullcalendat/list
	@nivo/core @nivo/pie @nivo/line @nivo/geo
	```
	* delete template apps:
		setupTest.js
		reportWebVitals.js
		logo.svg
		App.test.js
		App.css
	* in index.js:
		delete the lowest comment and tag
		delete report web vital line
		change upper case App ==> app
		```
		import { BrowserRouter } from "react-router-dom";
		```
		wrap <BrowserRouter> around <App />

	* in App.js:
		delete <header> tag
		delte top 2 import lines
		change upper case App ==> app
	 