#[doc = "Register `LSMRWDIS` reader"]
pub struct R(crate::R<LSMRWDIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSMRWDIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSMRWDIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSMRWDIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSMRWDIS` writer"]
pub struct W(crate::W<LSMRWDIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSMRWDIS_SPEC>;
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
impl From<crate::W<LSMRWDIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSMRWDIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTDIS` reader - WDT Operate Clock Control"]
pub type WDTDIS_R = crate::BitReader<WDTDIS_A>;
#[doc = "WDT Operate Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTDIS_A {
    #[doc = "0: WDT operates as normal"]
    _0 = 0,
    #[doc = "1: Stop the WDT clock and register R/W clock"]
    _1 = 1,
}
impl From<WDTDIS_A> for bool {
    #[inline(always)]
    fn from(variant: WDTDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl WDTDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTDIS_A {
        match self.bits {
            false => WDTDIS_A::_0,
            true => WDTDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WDTDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WDTDIS_A::_1
    }
}
#[doc = "Field `WDTDIS` writer - WDT Operate Clock Control"]
pub type WDTDIS_W<'a, const O: u8> = crate::BitWriter<'a, u16, LSMRWDIS_SPEC, WDTDIS_A, O>;
impl<'a, const O: u8> WDTDIS_W<'a, O> {
    #[doc = "WDT operates as normal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WDTDIS_A::_0)
    }
    #[doc = "Stop the WDT clock and register R/W clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WDTDIS_A::_1)
    }
}
#[doc = "Field `IWDTIDS` reader - IWDT Register Clock Control"]
pub type IWDTIDS_R = crate::BitReader<IWDTIDS_A>;
#[doc = "IWDT Register Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDTIDS_A {
    #[doc = "0: IWDT operates as normal"]
    _0 = 0,
    #[doc = "1: Stop the IWDT register R/W clock"]
    _1 = 1,
}
impl From<IWDTIDS_A> for bool {
    #[inline(always)]
    fn from(variant: IWDTIDS_A) -> Self {
        variant as u8 != 0
    }
}
impl IWDTIDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWDTIDS_A {
        match self.bits {
            false => IWDTIDS_A::_0,
            true => IWDTIDS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IWDTIDS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IWDTIDS_A::_1
    }
}
#[doc = "Field `IWDTIDS` writer - IWDT Register Clock Control"]
pub type IWDTIDS_W<'a, const O: u8> = crate::BitWriter<'a, u16, LSMRWDIS_SPEC, IWDTIDS_A, O>;
impl<'a, const O: u8> IWDTIDS_W<'a, O> {
    #[doc = "IWDT operates as normal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IWDTIDS_A::_0)
    }
    #[doc = "Stop the IWDT register R/W clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IWDTIDS_A::_1)
    }
}
#[doc = "Field `WREN` reader - Write Enable for bits \\[2:0\\]"]
pub type WREN_R = crate::BitReader<WREN_A>;
#[doc = "Write Enable for bits \\[2:0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WREN_A {
    #[doc = "0: Write protect for bits \\[2:0\\]"]
    _0 = 0,
    #[doc = "1: Write enable for bits \\[2:0\\]"]
    _1 = 1,
}
impl From<WREN_A> for bool {
    #[inline(always)]
    fn from(variant: WREN_A) -> Self {
        variant as u8 != 0
    }
}
impl WREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WREN_A {
        match self.bits {
            false => WREN_A::_0,
            true => WREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WREN_A::_1
    }
}
#[doc = "Field `WREN` writer - Write Enable for bits \\[2:0\\]"]
pub type WREN_W<'a, const O: u8> = crate::BitWriter<'a, u16, LSMRWDIS_SPEC, WREN_A, O>;
impl<'a, const O: u8> WREN_W<'a, O> {
    #[doc = "Write protect for bits \\[2:0\\]"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WREN_A::_0)
    }
    #[doc = "Write enable for bits \\[2:0\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WREN_A::_1)
    }
}
#[doc = "Field `PRKEY` writer - LSMRWDIS Key Code"]
pub type PRKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, LSMRWDIS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 1 - WDT Operate Clock Control"]
    #[inline(always)]
    pub fn wdtdis(&self) -> WDTDIS_R {
        WDTDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IWDT Register Clock Control"]
    #[inline(always)]
    pub fn iwdtids(&self) -> IWDTIDS_R {
        IWDTIDS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Write Enable for bits \\[2:0\\]"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - WDT Operate Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn wdtdis(&mut self) -> WDTDIS_W<1> {
        WDTDIS_W::new(self)
    }
    #[doc = "Bit 2 - IWDT Register Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn iwdtids(&mut self) -> IWDTIDS_W<2> {
        IWDTIDS_W::new(self)
    }
    #[doc = "Bit 7 - Write Enable for bits \\[2:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn wren(&mut self) -> WREN_W<7> {
        WREN_W::new(self)
    }
    #[doc = "Bits 8:15 - LSMRWDIS Key Code"]
    #[inline(always)]
    #[must_use]
    pub fn prkey(&mut self) -> PRKEY_W<8> {
        PRKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Speed Module R/W Disable Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsmrwdis](index.html) module"]
pub struct LSMRWDIS_SPEC;
impl crate::RegisterSpec for LSMRWDIS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lsmrwdis::R](R) reader structure"]
impl crate::Readable for LSMRWDIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lsmrwdis::W](W) writer structure"]
impl crate::Writable for LSMRWDIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LSMRWDIS to value 0"]
impl crate::Resettable for LSMRWDIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
