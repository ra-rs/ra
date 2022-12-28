#[doc = "Register `XCR1` reader"]
pub struct R(crate::R<XCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XCR1` writer"]
pub struct W(crate::W<XCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XCR1_SPEC>;
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
impl From<crate::W<XCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCST` reader - Break Field output timer count start trigger"]
pub type TCST_R = crate::BitReader<TCST_A>;
#[doc = "Break Field output timer count start trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCST_A {
    #[doc = "0: Break Field output timer count stopped"]
    _0 = 0,
    #[doc = "1: Break Field output timer count start"]
    _1 = 1,
}
impl From<TCST_A> for bool {
    #[inline(always)]
    fn from(variant: TCST_A) -> Self {
        variant as u8 != 0
    }
}
impl TCST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCST_A {
        match self.bits {
            false => TCST_A::_0,
            true => TCST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCST_A::_1
    }
}
#[doc = "Field `TCST` writer - Break Field output timer count start trigger"]
pub type TCST_W<'a, const O: u8> = crate::BitWriter<'a, u32, XCR1_SPEC, TCST_A, O>;
impl<'a, const O: u8> TCST_W<'a, O> {
    #[doc = "Break Field output timer count stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCST_A::_0)
    }
    #[doc = "Break Field output timer count start"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCST_A::_1)
    }
}
#[doc = "Field `SDST` reader - Start Frame detection enable"]
pub type SDST_R = crate::BitReader<SDST_A>;
#[doc = "Start Frame detection enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDST_A {
    #[doc = "0: Start Frame/Break Field detection disabled"]
    _0 = 0,
    #[doc = "1: Start Frame/Break Field detection enabled"]
    _1 = 1,
}
impl From<SDST_A> for bool {
    #[inline(always)]
    fn from(variant: SDST_A) -> Self {
        variant as u8 != 0
    }
}
impl SDST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDST_A {
        match self.bits {
            false => SDST_A::_0,
            true => SDST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDST_A::_1
    }
}
#[doc = "Field `SDST` writer - Start Frame detection enable"]
pub type SDST_W<'a, const O: u8> = crate::BitWriter<'a, u32, XCR1_SPEC, SDST_A, O>;
impl<'a, const O: u8> SDST_W<'a, O> {
    #[doc = "Start Frame/Break Field detection disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDST_A::_0)
    }
    #[doc = "Start Frame/Break Field detection enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDST_A::_1)
    }
}
#[doc = "Field `BMEN` reader - Bit rate measurement enable"]
pub type BMEN_R = crate::BitReader<BMEN_A>;
#[doc = "Bit rate measurement enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMEN_A {
    #[doc = "0: Bit rate measurement disabled"]
    _0 = 0,
    #[doc = "1: Bit rate measurement enabled"]
    _1 = 1,
}
impl From<BMEN_A> for bool {
    #[inline(always)]
    fn from(variant: BMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BMEN_A {
        match self.bits {
            false => BMEN_A::_0,
            true => BMEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BMEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BMEN_A::_1
    }
}
#[doc = "Field `BMEN` writer - Bit rate measurement enable"]
pub type BMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, XCR1_SPEC, BMEN_A, O>;
impl<'a, const O: u8> BMEN_W<'a, O> {
    #[doc = "Bit rate measurement disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BMEN_A::_0)
    }
    #[doc = "Bit rate measurement enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BMEN_A::_1)
    }
}
#[doc = "Field `PCF1D` reader - Priority compare data for Control Field 1"]
pub type PCF1D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCF1D` writer - Priority compare data for Control Field 1"]
pub type PCF1D_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XCR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `SCF1D` reader - Secondary compare data for Control Field 1"]
pub type SCF1D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCF1D` writer - Secondary compare data for Control Field 1"]
pub type SCF1D_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XCR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `CF1CE` reader - Control Field 1 compare bit enable"]
pub type CF1CE_R = crate::FieldReader<u8, CF1CE_A>;
#[doc = "Control Field 1 compare bit enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CF1CE_A {
    #[doc = "0: Control Field 1 bit N compare disabled"]
    _0 = 0,
    #[doc = "1: Control Field 1 bit N compare enabled"]
    _1 = 1,
}
impl From<CF1CE_A> for u8 {
    #[inline(always)]
    fn from(variant: CF1CE_A) -> Self {
        variant as _
    }
}
impl CF1CE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CF1CE_A> {
        match self.bits {
            0 => Some(CF1CE_A::_0),
            1 => Some(CF1CE_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CF1CE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CF1CE_A::_1
    }
}
#[doc = "Field `CF1CE` writer - Control Field 1 compare bit enable"]
pub type CF1CE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XCR1_SPEC, u8, CF1CE_A, 8, O>;
impl<'a, const O: u8> CF1CE_W<'a, O> {
    #[doc = "Control Field 1 bit N compare disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CF1CE_A::_0)
    }
    #[doc = "Control Field 1 bit N compare enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CF1CE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Break Field output timer count start trigger"]
    #[inline(always)]
    pub fn tcst(&self) -> TCST_R {
        TCST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Start Frame detection enable"]
    #[inline(always)]
    pub fn sdst(&self) -> SDST_R {
        SDST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit rate measurement enable"]
    #[inline(always)]
    pub fn bmen(&self) -> BMEN_R {
        BMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Priority compare data for Control Field 1"]
    #[inline(always)]
    pub fn pcf1d(&self) -> PCF1D_R {
        PCF1D_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Secondary compare data for Control Field 1"]
    #[inline(always)]
    pub fn scf1d(&self) -> SCF1D_R {
        SCF1D_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Control Field 1 compare bit enable"]
    #[inline(always)]
    pub fn cf1ce(&self) -> CF1CE_R {
        CF1CE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Break Field output timer count start trigger"]
    #[inline(always)]
    #[must_use]
    pub fn tcst(&mut self) -> TCST_W<0> {
        TCST_W::new(self)
    }
    #[doc = "Bit 4 - Start Frame detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdst(&mut self) -> SDST_W<4> {
        SDST_W::new(self)
    }
    #[doc = "Bit 5 - Bit rate measurement enable"]
    #[inline(always)]
    #[must_use]
    pub fn bmen(&mut self) -> BMEN_W<5> {
        BMEN_W::new(self)
    }
    #[doc = "Bits 8:15 - Priority compare data for Control Field 1"]
    #[inline(always)]
    #[must_use]
    pub fn pcf1d(&mut self) -> PCF1D_W<8> {
        PCF1D_W::new(self)
    }
    #[doc = "Bits 16:23 - Secondary compare data for Control Field 1"]
    #[inline(always)]
    #[must_use]
    pub fn scf1d(&mut self) -> SCF1D_W<16> {
        SCF1D_W::new(self)
    }
    #[doc = "Bits 24:31 - Control Field 1 compare bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf1ce(&mut self) -> CF1CE_W<24> {
        CF1CE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Simple LIN Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xcr1](index.html) module"]
pub struct XCR1_SPEC;
impl crate::RegisterSpec for XCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xcr1::R](R) reader structure"]
impl crate::Readable for XCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xcr1::W](W) writer structure"]
impl crate::Writable for XCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XCR1 to value 0"]
impl crate::Resettable for XCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
