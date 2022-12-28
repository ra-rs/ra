#[doc = "Register `SVDCT` reader"]
pub struct R(crate::R<SVDCT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SVDCT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SVDCT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SVDCT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SVDCT` writer"]
pub struct W(crate::W<SVDCT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SVDCT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SVDCT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SVDCT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDCR` reader - Transfar Device Characteristic Register"]
pub type TDCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDCR` writer - Transfar Device Characteristic Register"]
pub type TDCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SVDCT_SPEC, u8, u8, 8, O>;
#[doc = "Field `TBCR0` reader - Max Data Speed Limitation"]
pub type TBCR0_R = crate::BitReader<TBCR0_A>;
#[doc = "Max Data Speed Limitation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBCR0_A {
    #[doc = "0: No Limitation"]
    _0 = 0,
    #[doc = "1: Limitation"]
    _1 = 1,
}
impl From<TBCR0_A> for bool {
    #[inline(always)]
    fn from(variant: TBCR0_A) -> Self {
        variant as u8 != 0
    }
}
impl TBCR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBCR0_A {
        match self.bits {
            false => TBCR0_A::_0,
            true => TBCR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TBCR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TBCR0_A::_1
    }
}
#[doc = "Field `TBCR0` writer - Max Data Speed Limitation"]
pub type TBCR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SVDCT_SPEC, TBCR0_A, O>;
impl<'a, const O: u8> TBCR0_W<'a, O> {
    #[doc = "No Limitation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TBCR0_A::_0)
    }
    #[doc = "Limitation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TBCR0_A::_1)
    }
}
#[doc = "Field `TBCR1` reader - IBI Request Capable"]
pub type TBCR1_R = crate::BitReader<TBCR1_A>;
#[doc = "IBI Request Capable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBCR1_A {
    #[doc = "0: Not Capable"]
    _0 = 0,
    #[doc = "1: Capable"]
    _1 = 1,
}
impl From<TBCR1_A> for bool {
    #[inline(always)]
    fn from(variant: TBCR1_A) -> Self {
        variant as u8 != 0
    }
}
impl TBCR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBCR1_A {
        match self.bits {
            false => TBCR1_A::_0,
            true => TBCR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TBCR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TBCR1_A::_1
    }
}
#[doc = "Field `TBCR1` writer - IBI Request Capable"]
pub type TBCR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SVDCT_SPEC, TBCR1_A, O>;
impl<'a, const O: u8> TBCR1_W<'a, O> {
    #[doc = "Not Capable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TBCR1_A::_0)
    }
    #[doc = "Capable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TBCR1_A::_1)
    }
}
#[doc = "Field `TBCR2` reader - IBI Payload"]
pub type TBCR2_R = crate::BitReader<TBCR2_A>;
#[doc = "IBI Payload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBCR2_A {
    #[doc = "0: No data byte follows the accepted IBI."]
    _0 = 0,
    #[doc = "1: Mandatory one or more data bytes follow the accepted IBI. Data byte continuation is indicated by T-Bit."]
    _1 = 1,
}
impl From<TBCR2_A> for bool {
    #[inline(always)]
    fn from(variant: TBCR2_A) -> Self {
        variant as u8 != 0
    }
}
impl TBCR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBCR2_A {
        match self.bits {
            false => TBCR2_A::_0,
            true => TBCR2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TBCR2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TBCR2_A::_1
    }
}
#[doc = "Field `TBCR2` writer - IBI Payload"]
pub type TBCR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SVDCT_SPEC, TBCR2_A, O>;
impl<'a, const O: u8> TBCR2_W<'a, O> {
    #[doc = "No data byte follows the accepted IBI."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TBCR2_A::_0)
    }
    #[doc = "Mandatory one or more data bytes follow the accepted IBI. Data byte continuation is indicated by T-Bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TBCR2_A::_1)
    }
}
#[doc = "Field `TBCR3` reader - Offline Capable"]
pub type TBCR3_R = crate::BitReader<TBCR3_A>;
#[doc = "Offline Capable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBCR3_A {
    #[doc = "0: Device will always respond to I3C bus commands."]
    _0 = 0,
    #[doc = "1: Device will not always respond to I3C bus commands."]
    _1 = 1,
}
impl From<TBCR3_A> for bool {
    #[inline(always)]
    fn from(variant: TBCR3_A) -> Self {
        variant as u8 != 0
    }
}
impl TBCR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBCR3_A {
        match self.bits {
            false => TBCR3_A::_0,
            true => TBCR3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TBCR3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TBCR3_A::_1
    }
}
#[doc = "Field `TBCR3` writer - Offline Capable"]
pub type TBCR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SVDCT_SPEC, TBCR3_A, O>;
impl<'a, const O: u8> TBCR3_W<'a, O> {
    #[doc = "Device will always respond to I3C bus commands."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TBCR3_A::_0)
    }
    #[doc = "Device will not always respond to I3C bus commands."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TBCR3_A::_1)
    }
}
#[doc = "Field `TBCR76` reader - Device Role"]
pub type TBCR76_R = crate::FieldReader<u8, TBCR76_A>;
#[doc = "Device Role\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TBCR76_A {
    #[doc = "0: I3C Slave"]
    _00 = 0,
    #[doc = "1: I3C Master"]
    _01 = 1,
    #[doc = "2: Reserved for future definition by MIPI Sensor WG"]
    _10 = 2,
    #[doc = "3: Reserved for future definition by MIPI Sensor WG"]
    _11 = 3,
}
impl From<TBCR76_A> for u8 {
    #[inline(always)]
    fn from(variant: TBCR76_A) -> Self {
        variant as _
    }
}
impl TBCR76_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBCR76_A {
        match self.bits {
            0 => TBCR76_A::_00,
            1 => TBCR76_A::_01,
            2 => TBCR76_A::_10,
            3 => TBCR76_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TBCR76_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TBCR76_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TBCR76_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TBCR76_A::_11
    }
}
#[doc = "Field `TBCR76` writer - Device Role"]
pub type TBCR76_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SVDCT_SPEC, u8, TBCR76_A, 2, O>;
impl<'a, const O: u8> TBCR76_W<'a, O> {
    #[doc = "I3C Slave"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TBCR76_A::_00)
    }
    #[doc = "I3C Master"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TBCR76_A::_01)
    }
    #[doc = "Reserved for future definition by MIPI Sensor WG"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TBCR76_A::_10)
    }
    #[doc = "Reserved for future definition by MIPI Sensor WG"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TBCR76_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:7 - Transfar Device Characteristic Register"]
    #[inline(always)]
    pub fn tdcr(&self) -> TDCR_R {
        TDCR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Max Data Speed Limitation"]
    #[inline(always)]
    pub fn tbcr0(&self) -> TBCR0_R {
        TBCR0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IBI Request Capable"]
    #[inline(always)]
    pub fn tbcr1(&self) -> TBCR1_R {
        TBCR1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IBI Payload"]
    #[inline(always)]
    pub fn tbcr2(&self) -> TBCR2_R {
        TBCR2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Offline Capable"]
    #[inline(always)]
    pub fn tbcr3(&self) -> TBCR3_R {
        TBCR3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Device Role"]
    #[inline(always)]
    pub fn tbcr76(&self) -> TBCR76_R {
        TBCR76_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transfar Device Characteristic Register"]
    #[inline(always)]
    #[must_use]
    pub fn tdcr(&mut self) -> TDCR_W<0> {
        TDCR_W::new(self)
    }
    #[doc = "Bit 8 - Max Data Speed Limitation"]
    #[inline(always)]
    #[must_use]
    pub fn tbcr0(&mut self) -> TBCR0_W<8> {
        TBCR0_W::new(self)
    }
    #[doc = "Bit 9 - IBI Request Capable"]
    #[inline(always)]
    #[must_use]
    pub fn tbcr1(&mut self) -> TBCR1_W<9> {
        TBCR1_W::new(self)
    }
    #[doc = "Bit 10 - IBI Payload"]
    #[inline(always)]
    #[must_use]
    pub fn tbcr2(&mut self) -> TBCR2_W<10> {
        TBCR2_W::new(self)
    }
    #[doc = "Bit 11 - Offline Capable"]
    #[inline(always)]
    #[must_use]
    pub fn tbcr3(&mut self) -> TBCR3_W<11> {
        TBCR3_W::new(self)
    }
    #[doc = "Bits 14:15 - Device Role"]
    #[inline(always)]
    #[must_use]
    pub fn tbcr76(&mut self) -> TBCR76_W<14> {
        TBCR76_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Device Characteristic Table Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [svdct](index.html) module"]
pub struct SVDCT_SPEC;
impl crate::RegisterSpec for SVDCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [svdct::R](R) reader structure"]
impl crate::Readable for SVDCT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [svdct::W](W) writer structure"]
impl crate::Writable for SVDCT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SVDCT to value 0"]
impl crate::Resettable for SVDCT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
