{\rtf1\ansi\ansicpg1252\cocoartf1671\cocoasubrtf600
{\fonttbl\f0\fswiss\fcharset0 Helvetica;}
{\colortbl;\red255\green255\blue255;}
{\*\expandedcolortbl;;}
\margl1440\margr1440\vieww10800\viewh8400\viewkind0
\pard\tx720\tx1440\tx2160\tx2880\tx3600\tx4320\tx5040\tx5760\tx6480\tx7200\tx7920\tx8640\pardirnatural\partightenfactor0

\f0\fs24 \cf0 use super::ColorScheme;\
\
pub struct Green;\
\
impl ColorScheme for Green \{\
\
    fn getcolor(&self, hue: u32) -> u32 \{\
        if hue == 63 \{\
            return 0x16FE3D;\
        \}\
        let offset: u32 = hue % 16 * 16;\
        match hue / 16 \{\
            0 => 0x638A45 | offset << 8,\
            1 => 0x88C0B2 | 255 - offset << 16, \
            2 => 0x5A9B7E | offset,\
            3 => 0x81F05C | 255 - offset << 8,\
             => unreachable!(),\
        \}\
    \}\
\}}