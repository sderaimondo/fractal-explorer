{\rtf1\ansi\ansicpg1252\cocoartf1671\cocoasubrtf600
{\fonttbl\f0\fnil\fcharset0 HelveticaNeue;}
{\colortbl;\red255\green255\blue255;\red212\green213\blue214;\red5\green5\blue6;}
{\*\expandedcolortbl;;\cssrgb\c86275\c86667\c87059;\cssrgb\c1569\c1569\c1961\c6667;}
\margl1440\margr1440\vieww50700\viewh29340\viewkind0
\deftab720
\pard\pardeftab720\sl360\partightenfactor0

\f0\fs32 \cf2 \cb3 \expnd0\expndtw0\kerning0
use super::ColorScheme;\
\
pub struct Red;\
\
impl ColorScheme for Red \{\
\
    fn getcolor(&self, hue: u32) -> u32 \{\
        if hue == 63 \{\
            return 0xD0489A;\
        \}\
        let offset: u32 = hue % 16 * 16;\
        match hue / 16 \{\
            0 => 0x8A4563 | offset << 8,\
            1 => 0xC088B2 | 255 - offset << 16, \
            2 => 0x9B5A7E | offset,\
            3 => 0xE5B7E4 | 255 - offset << 8,\
             => unreachable!(),\
        \}\
    \}\
\}}