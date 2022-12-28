#[doc = "Register `MSDCT%s` reader"]
pub struct R(crate::R<MSDCT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSDCT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSDCT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSDCT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSDCT%s` writer"]
pub struct W(crate::W<MSDCT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSDCT_SPEC>;
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
impl From<crate::W<MSDCT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSDCT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RBCR0` reader - Max Data Speed Limitation"]
pub type RBCR0_R = crate::BitReader<RBCR0_A>;
#[doc = "Max Data Speed Limitation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBCR0_A {
    #[doc = "0: No Limitation"]
    _0 = 0,
    #[doc = "1: Limitation"]
    _1 = 1,
}
impl From<RBCR0_A> for bool {
    #[inline(always)]
    fn from(variant: RBCR0_A) -> Self {
        variant as u8 != 0
    }
}
impl RBCR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBCR0_A {
        match self.bits {
            false => RBCR0_A::_0,
            true => RBCR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RBCR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RBCR0_A::_1
    }
}
#[doc = "Field `RBCR0` writer - Max Data Speed Limitation"]
pub type RBCR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSDCT_SPEC, RBCR0_A, O>;
impl<'a, const O: u8> RBCR0_W<'a, O> {
    #[doc = "No Limitation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RBCR0_A::_0)
    }
    #[doc = "Limitation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RBCR0_A::_1)
    }
}
#[doc = "Field `RBCR1` reader - IBI Request Capable"]
pub type RBCR1_R = crate::BitReader<RBCR1_A>;
#[doc = "IBI Request Capable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBCR1_A {
    #[doc = "0: Not Capable"]
    _0 = 0,
    #[doc = "1: Capable"]
    _1 = 1,
}
impl From<RBCR1_A> for bool {
    #[inline(always)]
    fn from(variant: RBCR1_A) -> Self {
        variant as u8 != 0
    }
}
impl RBCR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBCR1_A {
        match self.bits {
            false => RBCR1_A::_0,
            true => RBCR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RBCR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RBCR1_A::_1
    }
}
#[doc = "Field `RBCR1` writer - IBI Request Capable"]
pub type RBCR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSDCT_SPEC, RBCR1_A, O>;
impl<'a, const O: u8> RBCR1_W<'a, O> {
    #[doc = "Not Capable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RBCR1_A::_0)
    }
    #[doc = "Capable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RBCR1_A::_1)
    }
}
#[doc = "Field `RBCR2` reader - IBI Payload"]
pub type RBCR2_R = crate::BitReader<RBCR2_A>;
#[doc = "IBI Payload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBCR2_A {
    #[doc = "0: No data byte follows the accepted IBI."]
    _0 = 0,
    #[doc = "1: Mandatory one or more data bytes follow the accepted IBI. Data byte continuation is indicated by T-Bit."]
    _1 = 1,
}
impl From<RBCR2_A> for bool {
    #[inline(always)]
    fn from(variant: RBCR2_A) -> Self {
        variant as u8 != 0
    }
}
impl RBCR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBCR2_A {
        match self.bits {
            false => RBCR2_A::_0,
            true => RBCR2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RBCR2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RBCR2_A::_1
    }
}
#[doc = "Field `RBCR2` writer - IBI Payload"]
pub type RBCR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSDCT_SPEC, RBCR2_A, O>;
impl<'a, const O: u8> RBCR2_W<'a, O> {
    #[doc = "No data byte follows the accepted IBI."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RBCR2_A::_0)
    }
    #[doc = "Mandatory one or more data bytes follow the accepted IBI. Data byte continuation is indicated by T-Bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RBCR2_A::_1)
    }
}
#[doc = "Field `RBCR3` reader - Offline Capable"]
pub type RBCR3_R = crate::BitReader<RBCR3_A>;
#[doc = "Offline Capable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBCR3_A {
    #[doc = "0: Device will always respond to I3C bus commands."]
    _0 = 0,
    #[doc = "1: Device will not always respond to I3C bus commands."]
    _1 = 1,
}
impl From<RBCR3_A> for bool {
    #[inline(always)]
    fn from(variant: RBCR3_A) -> Self {
        variant as u8 != 0
    }
}
impl RBCR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBCR3_A {
        match self.bits {
            false => RBCR3_A::_0,
            true => RBCR3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RBCR3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RBCR3_A::_1
    }
}
#[doc = "Field `RBCR3` writer - Offline Capable"]
pub type RBCR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSDCT_SPEC, RBCR3_A, O>;
impl<'a, const O: u8> RBCR3_W<'a, O> {
    #[doc = "Device will always respond to I3C bus commands."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RBCR3_A::_0)
    }
    #[doc = "Device will not always respond to I3C bus commands."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RBCR3_A::_1)
    }
}
#[doc = "Field `RBCR76` reader - Device Role"]
pub type RBCR76_R = crate::FieldReader<u8, RBCR76_A>;
#[doc = "Device Role\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RBCR76_A {
    #[doc = "0: I3C Slave"]
    _00 = 0,
    #[doc = "1: I3C Master"]
    _01 = 1,
}
impl From<RBCR76_A> for u8 {
    #[inline(always)]
    fn from(variant: RBCR76_A) -> Self {
        variant as _
    }
}
impl RBCR76_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RBCR76_A> {
        match self.bits {
            0 => Some(RBCR76_A::_00),
            1 => Some(RBCR76_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RBCR76_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RBCR76_A::_01
    }
}
#[doc = "Field `RBCR76` writer - Device Role"]
pub type RBCR76_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MSDCT_SPEC, u8, RBCR76_A, 2, O>;
impl<'a, const O: u8> RBCR76_W<'a, O> {
    #[doc = "I3C Slave"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RBCR76_A::_00)
    }
    #[doc = "I3C Master"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RBCR76_A::_01)
    }
}
impl R {
    #[doc = "Bit 8 - Max Data Speed Limitation"]
    #[inline(always)]
    pub fn rbcr0(&self) -> RBCR0_R {
        RBCR0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IBI Request Capable"]
    #[inline(always)]
    pub fn rbcr1(&self) -> RBCR1_R {
        RBCR1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IBI Payload"]
    #[inline(always)]
    pub fn rbcr2(&self) -> RBCR2_R {
        RBCR2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Offline Capable"]
    #[inline(always)]
    pub fn rbcr3(&self) -> RBCR3_R {
        RBCR3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Device Role"]
    #[inline(always)]
    pub fn rbcr76(&self) -> RBCR76_R {
        RBCR76_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Max Data Speed Limitation"]
    #[inline(always)]
    #[must_use]
    pub fn rbcr0(&mut self) -> RBCR0_W<8> {
        RBCR0_W::new(self)
    }
    #[doc = "Bit 9 - IBI Request Capable"]
    #[inline(always)]
    #[must_use]
    pub fn rbcr1(&mut self) -> RBCR1_W<9> {
        RBCR1_W::new(self)
    }
    #[doc = "Bit 10 - IBI Payload"]
    #[inline(always)]
    #[must_use]
    pub fn rbcr2(&mut self) -> RBCR2_W<10> {
        RBCR2_W::new(self)
    }
    #[doc = "Bit 11 - Offline Capable"]
    #[inline(always)]
    #[must_use]
    pub fn rbcr3(&mut self) -> RBCR3_W<11> {
        RBCR3_W::new(self)
    }
    #[doc = "Bits 14:15 - Device Role"]
    #[inline(always)]
    #[must_use]
    pub fn rbcr76(&mut self) -> RBCR76_W<14> {
        RBCR76_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Device Characteristic Table Register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msdct](index.html) module"]
pub struct MSDCT_SPEC;
impl crate::RegisterSpec for MSDCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msdct::R](R) reader structure"]
impl crate::Readable for MSDCT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msdct::W](W) writer structure"]
impl crate::Writable for MSDCT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSDCT%s to value 0"]
impl crate::Resettable for MSDCT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
