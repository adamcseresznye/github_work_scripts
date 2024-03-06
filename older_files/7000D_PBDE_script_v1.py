#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Fri Feb  4 19:32:26 2022

@author: adamcseresznye
"""

%reset -sf
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import os
import glob
import re

###################################################################
############## PART I: Read in files and tidy up ##################
###################################################################

#response = pd.read_csv('/Users/adamcseresznye/Desktop/Work/eWaste/Batch_III/220217_e-waste-BFRs-batch3_Response.csv', index_col=('Response_ID'))
#concentration = pd.read_csv('/Users/adamcseresznye/Desktop/Work/eWaste/Batch_III/220217_e-waste-BFRs-batch3_Concentration.csv', index_col=('Response_ID'))


##Read in file exported from Masshunter:
df = pd.read_csv('/Users/adamcseresznye/Desktop/Work/eWaste/Batch_III/Result/220217_e-waste-BFRs-batchIII.csv')

#drop columns not needed:
df = df[df.loc[:,'Unnamed: 2'] == 'Sample']
df.drop(['Unnamed: 1','Unnamed: 2','Unnamed: 3'] , axis =1, inplace = True)
df.set_index('Sample', inplace = True)

#separate dataframe into three different ones: 
concentration_native = df.iloc[:,0::3]
area_native = df.iloc[:,1::3]
area_IS = df.iloc[:,2::3]

#generate comoound names by removing unnecesary parts:
concentration_native.columns = concentration_native.columns.str.replace(' Results', '')
concentration_native.columns = concentration_native.columns.str.replace('-', '_')

#rename compound names for area_native df:
area_native.columns = concentration_native.columns

#Transpose dataframes
area_native = area_native.T
concentration_native = concentration_native.T
area_IS = area_IS.T

#Remove duplicate IS area values:
area_IS = area_IS.iloc[[0,7,8,9,10]]

#join area_IS and area_native:
response = pd.concat([area_IS, area_native], axis = 0)

response.index.names = ['Response_ID']

response.rename(index={"BDE-103 IS (ISTD) Results": 'BDE_103_IS',
                       "BDE-128 IS (ISTD) Results" : "BDE_128_IS",
                       "syn-DP IS (ISTD) Results" : "13C_syn_DP",
                       "anti-DP IS (ISTD) Results" : "13C_anti_DP",
                       "BDE-209 IS (ISTD) Results" : "13C_BDE209",
                       "CB_207" : "CB_207_RS"}, inplace = True)

response = response.reindex(['BDE_103_IS', 'BDE_128_IS', '13C_syn_DP', 
            '13C_anti_DP', '13C_BDE209','CB_207_RS',
            'BDE_28','BDE_47','BDE_99','BDE_100', 
            'BDE_153','BDE_154','BDE_183', 
            'BDE_209','anti_DP','syn_DP'])

#Set concentration values for internal standards:
IS_table = pd.DataFrame({'Index': {0: 'BDE_103_IS', 1: 'BDE_128_IS', 2: '13C_syn_DP',
                             3: '13C_anti_DP', 4: '13C_BDE209', 5: 'CB_207_RS'},
                   'mass': {0: 5000, 1: 5000, 2: 5000,
                            3: 5000 ,4: 6250, 5: 2500}}) \
    .set_index('Index')

#Fill up the dataframe with values so that it has same dimension as the concentration_native dataframe
for i in list(range(concentration_native.shape[1] - 1)):
    IS_table[i] = IS_table.mass
    i += 1

#change column names:
IS_table.set_axis(concentration_native.columns, axis = 1, inplace=True)

#join IS_table and concentration_native:
concentration = pd.concat([IS_table, concentration_native], axis = 0)

concentration.index.names = ['Response_ID']

#Remove the duplicate CB_207:
concentration.drop('CB_207', axis = 0, inplace = True)

#Reorder rows:
concentration = concentration.reindex(['BDE_103_IS', 'BDE_128_IS', '13C_syn_DP', 
            '13C_anti_DP', '13C_BDE209','CB_207_RS',
            'BDE_28','BDE_47','BDE_99','BDE_100', 
            'BDE_153','BDE_154','BDE_183', 
            'BDE_209','anti_DP','syn_DP'])


###################################################################
##### PART II: Calculating response factor and recovery ###########
###################################################################

concentration_filter = response.filter(regex = 'IS-RS') \
        .iloc[:6,:]
concentration_filter.index.name = 'Response_ID'


#Set concentration values for internal standards:
df = pd.DataFrame({'Index': {0: 'BDE_103_IS', 1: 'BDE_128_IS', 2: '13C_syn_DP',
                             3: '13C_anti_DP', 4: '13C_BDE209', 5: 'CB_207_RS'},
                   'mass': {0: 5000, 1: 5000, 2: 5000,
                            3: 5000 ,4: 6250, 5: 2500}}) \
    .set_index('Index')

#Fill up the dataframe with values so that it has same dimension as the concentration_filter dataframe
for i in list(range(concentration_filter.shape[1] - 1)):
    df[i] = df.mass
    i += 1

#change column names:
df.set_axis(concentration_filter.columns, axis = 1, inplace=True)
#put dataframes in a list so that we can concatanate them:
frames = [concentration_filter,df]
RF_df = pd.concat(frames, axis = 0,copy = False) \
    .apply(pd.to_numeric)

#Calculate the RRF for the ISs:
d = pd.DataFrame()
for i in RF_df:
    temp = pd.DataFrame(
        {
            'BDE_103_IS_RRF': [(RF_df[i][0] * RF_df[i][11]) / (RF_df[i][5] * RF_df[i][6])],
            'BDE_128_IS_RRF': [(RF_df[i][1] * RF_df[i][11]) / (RF_df[i][5] * RF_df[i][7])],
            '13C_syn_DP_RRF': [(RF_df[i][2] * RF_df[i][11]) / (RF_df[i][5] * RF_df[i][8])],
            '13C_anti_DP_RRF': [(RF_df[i][3] * RF_df[i][11]) / (RF_df[i][5] * RF_df[i][9])],
            '13C_BDE209_RRF': [(RF_df[i][4] * RF_df[i][11]) / (RF_df[i][5] * RF_df[i][10])]
    
        }
        )
    d = pd.concat([d, temp], ignore_index = True)

#Calculate the average RRF for the ISs:
average_RRF = []
for i in d:
    df = d[i].mean()
    average_RRF.append(df)
#Extract the average RRFs to a dataframe:
AVG_RRF = d.describe() \
    .loc['mean']

#Calculating expected recoveries:
no_IS = response.loc[:,~response.columns.str.contains('IS-RS')]\
    .iloc[:6,:]\
        .apply(pd.to_numeric)

#Calculating the actual amounts of ISs extracted from each samples:
expected_pg = pd.DataFrame()
for i in no_IS:
    temp = pd.DataFrame(
        {
           'BDE_103_IS_extracted_pg': [(no_IS.loc['BDE_103_IS'][i] * 2500 ) / (no_IS.loc['CB_207_RS'][i] * AVG_RRF.loc['BDE_103_IS_RRF'])],
           'BDE_128_IS_extracted_pg': [(no_IS.loc['BDE_128_IS'][i] * 2500 ) / (no_IS.loc['CB_207_RS'][i] * AVG_RRF.loc['BDE_128_IS_RRF'])],
           '13C_syn_DP_extracted_pg': [(no_IS.loc['13C_syn_DP'][i] * 2500 ) / (no_IS.loc['CB_207_RS'][i] * AVG_RRF.loc['13C_syn_DP_RRF'])],
           '13C_anti_DP_extracted_pg': [(no_IS.loc['13C_anti_DP'][i] * 2500 ) / (no_IS.loc['CB_207_RS'][i] * AVG_RRF.loc['13C_anti_DP_RRF'])],
           '13C_BDE209_extracted_pg': [(no_IS.loc['13C_BDE209'][i] * 2500 ) / (no_IS.loc['CB_207_RS'][i] * AVG_RRF.loc['13C_BDE209_RRF'])],
    
        }
        )
    expected_pg = pd.concat([expected_pg, temp], ignore_index = True)

#Create two series that contain the IS recoveries from all samples

recovery_BDE103 = pd.Series((expected_pg.loc[:,'BDE_103_IS_extracted_pg'] / 5000) * 100)
recovery_BDE128 = pd.Series((expected_pg.loc[:,'BDE_128_IS_extracted_pg'] / 5000) * 100)
recovery_synDP = pd.Series((expected_pg.loc[:,'13C_syn_DP_extracted_pg'] / 5000) * 100)
recovery_antiDP = pd.Series((expected_pg.loc[:,'13C_anti_DP_extracted_pg'] / 5000) * 100)
recovery_BDE209 = pd.Series((expected_pg.loc[:,'13C_BDE209_extracted_pg'] / 6500) * 100)

#zipping these two series together
Recovery = pd.DataFrame(list(zip(recovery_BDE103, recovery_BDE128, recovery_synDP,
                                 recovery_antiDP, recovery_BDE209)),
               columns =['BDE_103_IS', 'BDE_128_IS', 
                         '13C_syn_DP', '13C_anti_DP',
                         '13C_BDE209'])

#Assign sample names as indexes:
Recovery = Recovery.set_index(no_IS.columns)

#Summary of the Extraction efficiencies:
Recovery.describe()

fig = plt.figure()
ax = plt.subplot(111)
ax.boxplot(Recovery)
xticks = list(Recovery.columns)
ax.set_xticklabels(xticks, fontsize=10, rotation = -90)
ax.set_title('Recovery of internal standards in all samples measured')
ax.set_ylabel('% Recovery')

###################################################################
############## PART III: Calculating Correction factors ###########
###################################################################

#Get the pg values measured in the samples:
QC_pg = concentration.filter(regex = 'AMAP') \
        .iloc[6:,:] \
            .apply(pd.to_numeric)
QC_pg.index.name = 'Response_ID'        

#First, let's calculate the analyte concentrations measured in the blanks:
blank_filtered = concentration.filter(like = 'Blank' or 'blank') \
        .iloc[6:,:] \
            .apply(pd.to_numeric)
    
blank_filtered.index.name = 'Response_ID'            
        
#Calculate how many pg of analytes are there present in the samples (on average)
blank_values = []
for i in range(len(blank_filtered)):
    temp = blank_filtered.iloc[i,:].mean()
    blank_values.append(temp)

#Let's convert it to a pandas series
blank_values = pd.Series(blank_values, index = QC_pg.index) \
    .rename({'0' : 'blank_pg'})

#Get the blank corrected QC values in ng/ml:
QC_corrected_conc = (QC_pg.sub(blank_values, axis = 0)) / (0.5 * 1000)
#could be calculated like this too: ((QC_pg.T - blank_values).T)/ (0.5 * 1000)
#check out https://bit.ly/3utACIR

#Let's make a series that contains the theretical assigned values of analytes 
#in the QC
theoretical_assigned = {
    'BDE_28' : 0.225,
    'BDE_47' : 1.050,
    'BDE_99' : 0.498,
    'BDE_100': 0.340,
    'BDE_153': 0.371,
    'BDE_154': 0.748,
    'BDE_183': 0.409,
    'BDE_209': 0.981,
    'anti_DP': 1.000,
    'syn_DP' : 1.000
     }
theoretical_assigned = pd.Series(data=theoretical_assigned)

#Calculate the mean of blank corrected concentrations (ng/ml)
QC_corrected_conc_mean = []
for i in range(len(QC_pg)):
    temp = QC_corrected_conc.iloc[i,:].mean()
    QC_corrected_conc_mean.append(temp)

#Transform the list created in the previous step to a series:
QC_corrected_conc_mean = pd.Series(QC_corrected_conc_mean, index = theoretical_assigned.index)

#Calculate the correction factors and set values from CB 194 to gamma-HCH to 1
#these compounds are not present in the AMAP samples
correction_factor = theoretical_assigned / QC_corrected_conc_mean

#Set correction factor for BDE_183,anti_DP and syn_DP to 1:
correction_factor[[6,8,9]] = 1

#Let's plot the calculated correction factors:
fig = plt.figure()
ax1 = plt.subplot(2,3,2)
ax2 = plt.subplot(2,1,2)
fig.suptitle('Calculated correction factors', fontweight="bold")
ax1.boxplot(correction_factor)
ax2.bar(range(len(correction_factor)),correction_factor)
xticks = list(correction_factor.index)
ax1.set_xticklabels([])
ax2.set_xticklabels(xticks, fontsize=7,rotation = -90)
ax2.set_xticks(np.arange(min(list(range(len(xticks)))),max(list(range(len(xticks)))) + 1,1))
ax1.set_title('Boxplot of calculated correction factors', size=10)
ax2.set_title('Calculated correction factors for individual natives', size=10)
ax1.title.set_text('Boxplot of calculated correction factors:')
ax2.title.set_text('Calculated correction factors for individual analytes:')
ax1.set_ylabel('Correction factor', fontsize = 8)
ax2.set_ylabel('Correction factor', fontsize = 8)
ax1.tick_params(axis='both', which='major', labelsize=8)
ax2.tick_params(axis='both', which='major', labelsize=8)

###################################################################
########## PART IV: Calculating Sample Concentrations #############
###################################################################

#create dataframe that only contains the samples:
sample_concentration_pg = concentration.drop(list(concentration.filter(regex = '(IS-RS|blank|AMAP|Blank)')), 
                                             axis = 1) \
    .tail(concentration.shape[0] - 6) \
        .apply(pd.to_numeric)
sample_concentration_pg.index.name = 'Concentration_ID'  

#Let's calculate the corrected sample concentrations[pg]
new_index = theoretical_assigned.index
correction_factor.reindex(new_index)

corrected_sample_concentration_pg = sample_concentration_pg.sub(blank_values, axis = 0) \
    .mul(correction_factor, axis = 0)
    
#As one of the last steps, we have to imput the amount of sample extracted.
#Let's do this through a csv file.

sample_list = corrected_sample_concentration_pg.columns.tolist() 
    
#We can convert the list object to series and save it as csv:
sample_list_empty = pd.Series(sample_list) \
    .to_csv('sample_list_empty.csv', index = False)

#Read in the file and convert it to series:
sample_list_filled_in = pd.read_csv('/Users/adamcseresznye/Desktop/Work/eWaste/Batch_III/Result/sample_list_empty.csv', index_col = 0)\
    .squeeze()

#Divide the calculated sample concentrations [pg] with the amount of sample extracted.
#Now we get ng/ml
corrected_sample_concentration_ngml = corrected_sample_concentration_pg.div(sample_list_filled_in, axis = 1)

#Final step: replace negative ng/ml values with zeros:
corrected_sample_concentration_ngml[corrected_sample_concentration_ngml < 0] = 0

#Boxplot of the analyte concentrations per sample:
corrected_sample_concentration_ngml.plot.box(rot = 90, fontsize = 5,
                                             ylabel = 'c [pg/ml]',
                                             title = 'Concentrations according to samples')

#Sumarize the targets:
target_summary = corrected_sample_concentration_ngml.T.describe()

#Boxplot of the sample concentration per analyte:
corrected_sample_concentration_ngml.T.plot.box(rot = 90, ylabel = 'c [pg/ml]',
                                               title = 'Concentrations according to natives')


#################################################################
#################################################################
#################################################################

#Barplots without the 2022 AMAPs:
corrected_sample_concentration_ngml.loc[:,~corrected_sample_concentration_ngml.columns.str.contains('2022')].T.plot.box(rot = 90, ylabel = 'c [pg/ml]',
                                               title = 'Concentrations according to natives')
corrected_sample_concentration_ngml.loc[:,~corrected_sample_concentration_ngml.columns.str.contains('2022')].plot.box(rot = 90, ylabel = 'c [pg/ml]',
                                               title = 'Concentrations according to natives')

#subseting the respose table to plot the IS areas in all samples:
IS_area_to_plot = response.iloc[:6,:].T \
        .apply(pd.to_numeric)
IS_area_to_plot.plot.box(rot = 90, ylabel = 'Peak area',
                      title = 'Peak areas of internal standards in all samples')

#subseting the respose table to plot the native areas in all samples:
native_area_to_plot = response.iloc[6:,:].T \
        .apply(pd.to_numeric)
native_area_to_plot.plot.box(rot = 90, ylabel = 'Peak area',
                             title = 'Peak areas of natives in all samples')











    