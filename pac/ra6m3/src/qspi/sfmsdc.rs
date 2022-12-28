#[doc = "Register `SFMSDC` reader"]
pub struct R(crate::R<SFMSDC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFMSDC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFMSDC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFMSDC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFMSDC` writer"]
pub struct W(crate::W<SFMSDC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFMSDC_SPEC>;
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
impl From<crate::W<SFMSDC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFMSDC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFMDN` reader - Selection of the number of dummy cycles of Fast Read instructions"]
pub type SFMDN_R = crate::FieldReader<u8, SFMDN_A>;
#[doc = "Selection of the number of dummy cycles of Fast Read instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFMDN_A {
    #[doc = "0: Default dummy cycles of each instruction."]
    _0000 = 0,
}
impl From<SFMDN_A> for u8 {
    #[inline(always)]
    fn from(variant: SFMDN_A) -> Self {
        variant as _
    }
}
impl SFMDN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SFMDN_A> {
        match self.bits {
            0 => Some(SFMDN_A::_0000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == SFMDN_A::_0000
    }
}
#[doc = "Field `SFMDN` writer - Selection of the number of dummy cycles of Fast Read instructions"]
pub type SFMDN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SFMSDC_SPEC, u8, SFMDN_A, 4, O>;
impl<'a, const O: u8> SFMDN_W<'a, O> {
    #[doc = "Default dummy cycles of each instruction."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(SFMDN_A::_0000)
    }
}
#[doc = "Field `SFMXST` reader - XIP mode status"]
pub type SFMXST_R = crate::BitReader<SFMXST_A>;
#[doc = "XIP mode status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMXST_A {
    #[doc = "0: Normal (non-XIP) mode is operating"]
    _0 = 0,
    #[doc = "1: XIP mode is operating"]
    _1 = 1,
}
impl From<SFMXST_A> for bool {
    #[inline(always)]
    fn from(variant: SFMXST_A) -> Self {
        variant as u8 != 0
    }
}
impl SFMXST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMXST_A {
        match self.bits {
            false => SFMXST_A::_0,
            true => SFMXST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMXST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMXST_A::_1
    }
}
#[doc = "Field `SFMXEN` reader - XIP mode permission"]
pub type SFMXEN_R = crate::BitReader<SFMXEN_A>;
#[doc = "XIP mode permission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMXEN_A {
    #[doc = "0: XIP mode is prohibited"]
    _0 = 0,
    #[doc = "1: XIP mode is permitted"]
    _1 = 1,
}
impl From<SFMXEN_A> for bool {
    #[inline(always)]
    fn from(variant: SFMXEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SFMXEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMXEN_A {
        match self.bits {
            false => SFMXEN_A::_0,
            true => SFMXEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMXEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMXEN_A::_1
    }
}
#[doc = "Field `SFMXEN` writer - XIP mode permission"]
pub type SFMXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMSDC_SPEC, SFMXEN_A, O>;
impl<'a, const O: u8> SFMXEN_W<'a, O> {
    #[doc = "XIP mode is prohibited"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFMXEN_A::_0)
    }
    #[doc = "XIP mode is permitted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFMXEN_A::_1)
    }
}
#[doc = "Field `SFMXD` reader - Mode data for serial ROM. (Control XIP mode)"]
pub type SFMXD_R = crate::FieldReader<u8, SFMXD_A>;
#[doc = "Mode data for serial ROM. (Control XIP mode)\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFMXD_A {
    #[doc = "0: XIP mode is prohibited"]
    _0 = 0,
    #[doc = "1: XIP mode is permitted"]
    _1 = 1,
}
impl From<SFMXD_A> for u8 {
    #[inline(always)]
    fn from(variant: SFMXD_A) -> Self {
        variant as _
    }
}
impl SFMXD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SFMXD_A> {
        match self.bits {
            0 => Some(SFMXD_A::_0),
            1 => Some(SFMXD_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMXD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMXD_A::_1
    }
}
#[doc = "Field `SFMXD` writer - Mode data for serial ROM. (Control XIP mode)"]
pub type SFMXD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SFMSDC_SPEC, u8, SFMXD_A, 8, O>;
impl<'a, const O: u8> SFMXD_W<'a, O> {
    #[doc = "XIP mode is prohibited"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFMXD_A::_0)
    }
    #[doc = "XIP mode is permitted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFMXD_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Selection of the number of dummy cycles of Fast Read instructions"]
    #[inline(always)]
    pub fn sfmdn(&self) -> SFMDN_R {
        SFMDN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - XIP mode status"]
    #[inline(always)]
    pub fn sfmxst(&self) -> SFMXST_R {
        SFMXST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - XIP mode permission"]
    #[inline(always)]
    pub fn sfmxen(&self) -> SFMXEN_R {
        SFMXEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Mode data for serial ROM. (Control XIP mode)"]
    #[inline(always)]
    pub fn sfmxd(&self) -> SFMXD_R {
        SFMXD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selection of the number of dummy cycles of Fast Read instructions"]
    #[inline(always)]
    #[must_use]
    pub fn sfmdn(&mut self) -> SFMDN_W<0> {
        SFMDN_W::new(self)
    }
    #[doc = "Bit 7 - XIP mode permission"]
    #[inline(always)]
    #[must_use]
    pub fn sfmxen(&mut self) -> SFMXEN_W<7> {
        SFMXEN_W::new(self)
    }
    #[doc = "Bits 8:15 - Mode data for serial ROM. (Control XIP mode)"]
    #[inline(always)]
    #[must_use]
    pub fn sfmxd(&mut self) -> SFMXD_W<8> {
        SFMXD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dummy Cycle Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfmsdc](index.html) module"]
pub struct SFMSDC_SPEC;
impl crate::RegisterSpec for SFMSDC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfmsdc::R](R) reader structure"]
impl crate::Readable for SFMSDC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfmsdc::W](W) writer structure"]
impl crate::Writable for SFMSDC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFMSDC to value 0xff00"]
impl crate::Resettable for SFMSDC_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
