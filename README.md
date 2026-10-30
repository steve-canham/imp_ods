<h2>Imp_ods</h2>
A utility intended for periodic imports of NHS Organisational data, as made available by ODS (the Organisational Data Servie), processing that data and making a subset of it available for other systems.<br/>
Data can be downloaded by using the ODS's 'export packs' facilty, available at https://www.odsdatasearchandexport.nhs.uk/. The required files can be selected - usually as two or three distinct packs - and the download format (csv without headers) chosen. The csv files can then be extracted from the resultant downloaded zip file.<br/>
<br/>
Data is stored in a Postgres database called 'nhs', within a schema called ods. Data from 28 csv files are imported, listed below. 
In each case the name fields are put into Capitalised form - in the original ODS data they are all in upper case. This means that most words are in lower case with an initial capital letter, though abbreviations and acronyms that would normally all be in upper case are recognised and retained as upper case, while small words (like 'and', 'to', 'of' etc.) are all in lower case. This makes the data much easier to read and work with.<br/>
<br/>
Further processing of the data, together with integration with both Scottish data, and other sources of NHS related data, is planned for the future.<br/>
<br/>
The source files, and the destination database tables for each, are listed below. Figures in brackets refer to the number of records on 13/11/2025.<br/>
<br/>
<ul>
<li>Source: eauth.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.auths<br/>
<i>The high level national and regional geographic organisations (25).</i><br/><br/></li>

<li>Source: eccg.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.ccgs<br/>
<i>Clinical Commissioning Groups. Now defunct though many are now Sub-ICBs (320).</i><br/><br/></li>

<li>Source: eccgsite.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.ccg_sites<br/>
<i>Sites used by Clinical Commissioning Groups / Sub-ICBs (2,186).</i><br/><br/></li>

<li>Source: ecsu.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.csus<br/>
<i>Commissionning Support Units (e.g. data centres) - set up for CCGs but in some cases now used by ICBs (40).</i><br/><br/></li>

<li>Source: ecsusite.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.csu_sites<br/>
<i>Sites used by Commissionning Support Units (149).</i><br/><br/></li>

<li>Source: ect.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.care_trusts<br/>
<i>Care Trusts - a small number of joint NHS / LA trusts. Only 3 are listed as still active (11).</i><br/><br/></li>

<li>Source: ectsite.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.care_trust_sites<br/>
<i>Sites used by Care Trusts (996).</i><br/><br/></li>

<li>Source: ehospice.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.hospices<br/>
<i>List of hospice provision - some standalone, some within hospitals (322).</i><br/><br/></li>

<li>Source: eiom.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.iom_orgs<br/>
<i>Health organisations in the Isle of Man (15).</i><br/><br/></li>

<li>Source: enonnhs.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.non_nhs<br/>
<i>Non NHS organisations such as nursing homes, doctors practising privately, but not private healthcare providers (17,559).</i><br/><br/></li>

<li>Source: ensa.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.supp_agencies<br/>
<i>NHS Support agencies - often shared business or information services (198). </i><br/><br/></li>

<li>Source: eother.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.exec_agencies<br/>
<i>NHS Executive agencies - A wide variety of management organisations at different levels and with different scopes. Includes ICBs and government departments, but also pharmaceutical and pharmacy companies (991).</i><br/><br/></li>

<li>Source: epcmem.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.gpmem<br/>
<i>The linkage between GP practices and CCGs / LHBs - using codes only. A large proportion represent now defunct links (46,677).</i><br/><br/></li>

<li>Source: epcn.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.pcns<br/>
<i>Primary care Networks, each representing local clusters of GP practices (1,386).</i><br/><br/></li>

<li>Source: epcncorepartnerdetails.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.pcn_partners<br/>
<i>The surgeries within each Primary Care Network. Also includes the relevant Sub-ICB code (7,784).</i><br/><br/></li>

<li>Source: ephp.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.php_providers<br/>
<i>Private healthcare providers - a wide range of different types, including ophthalmic treatment and mental health provision (3,791).</i><br/><br/></li>

<li>Source: ephpsite.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.php_provider_sites<br/>
<i>Sites used by private healthcare providers (19,210).</i><br/><br/></li>

<li>Source: epraccur.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.gps<br/>
<i>GP surgeries - amongst other types of 'prescribing cost centres' (15,320).</i><br/><br/></li>

<li>Source: espha.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table ods.shas<br/>
<i>The NHS Special Health Authorities, i.e. specialist agencies, usually operating at national level (19).</i><br/><br/></li>

<li>Source: etr.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.trusts<br/>
<i>NHS trusts and foundation trusts, including many now defunct organisations (273).</i><br/><br/></li>

<li>Source: etreat.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.treat_centres<br/>
<i>Designated 'Treatment Centres', many within acute hospitals, many standalone specialist treatment units (211).</i><br/><br/></li>

<li>Source: ets.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.trust_sites<br/>
<i>Sites used by NHS trusts and foundation trusts. Many sites are included multiple times, presumed to represent different budgetary or prescribing lines (44,917).</i><br/><br/></li>

<li>Source: niorg.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.ni_orgs<br/>
<i>Health and social care trusts and commissioning groups in Northern Ireland (16).</i><br/><br/></li>

<li>Source: nlhscgpr.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.ni_gps_in_lhscg<br/>
<i>LInkage between Northern Irish GPs and NI commissioning groups - using codes only (321).</i><br/><br/></li>

<li>Source: npraccur.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.ni_gps<br/>
<i>Northern Irish GPs (321).</i><br/><br/></li>

<li>Source: succ.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.successions<br/>
<i>Links between organisations and their direct successors - using codes only (11,518).</i><br/><br/></li>

<li>Source: wlhb.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.wlhbs<br/>
<i>Welsh local health boards (7).</i><br/><br/></li>

<li>Source: wlhbsite.csv&nbsp;&nbsp;&nbsp;&nbsp;Destination Table: ods.wlhb_sites<br/>
<i>Sites used by Welsh local health boards, though parent organisation is not given (915).</i><br/><br/></li>

</ul>


