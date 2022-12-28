#[doc = "Register `DCPCFG` reader"]
pub struct R(crate::R<DCPCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCPCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCPCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCPCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCPCFG` writer"]
pub struct W(crate::W<DCPCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCPCFG_SPEC>;
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
impl From<crate::W<DCPCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCPCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIR` reader - Transfer Direction"]
pub type DIR_R = crate::BitReader<DIR_A>;
#[doc = "Transfer Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    #[doc = "0: Data receiving direction"]
    _0 = 0,
    #[doc = "1: Data transmitting direction"]
    _1 = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::_0,
            true => DIR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIR_A::_1
    }
}
#[doc = "Field `DIR` writer - Transfer Direction"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u16, DCPCFG_SPEC, DIR_A, O>;
impl<'a, const O: u8> DIR_W<'a, O> {
    #[doc = "Data receiving direction"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIR_A::_0)
    }
    #[doc = "Data transmitting direction"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIR_A::_1)
    }
}
#[doc = "Field `SHTNAK` reader - Pipe Blocking on End of Transfer"]
pub type SHTNAK_R = crate::BitReader<SHTNAK_A>;
#[doc = "Pipe Blocking on End of Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHTNAK_A {
    #[doc = "0: The pipe remains open after transfer ends."]
    _0 = 0,
    #[doc = "1: The pipe is blocked after transfer ends."]
    _1 = 1,
}
impl From<SHTNAK_A> for bool {
    #[inline(always)]
    fn from(variant: SHTNAK_A) -> Self {
        variant as u8 != 0
    }
}
impl SHTNAK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHTNAK_A {
        match self.bits {
            false => SHTNAK_A::_0,
            true => SHTNAK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHTNAK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHTNAK_A::_1
    }
}
#[doc = "Field `SHTNAK` writer - Pipe Blocking on End of Transfer"]
pub type SHTNAK_W<'a, const O: u8> = crate::BitWriter<'a, u16, DCPCFG_SPEC, SHTNAK_A, O>;
impl<'a, const O: u8> SHTNAK_W<'a, O> {
    #[doc = "The pipe remains open after transfer ends."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHTNAK_A::_0)
    }
    #[doc = "The pipe is blocked after transfer ends."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHTNAK_A::_1)
    }
}
#[doc = "Field `CNTMD` reader - Continuous Transfer Mode"]
pub type CNTMD_R = crate::BitReader<CNTMD_A>;
#[doc = "Continuous Transfer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTMD_A {
    #[doc = "0: Non-continuous transfer mode"]
    _0 = 0,
    #[doc = "1: Continuous transfer mode"]
    _1 = 1,
}
impl From<CNTMD_A> for bool {
    #[inline(always)]
    fn from(variant: CNTMD_A) -> Self {
        variant as u8 != 0
    }
}
impl CNTMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTMD_A {
        match self.bits {
            false => CNTMD_A::_0,
            true => CNTMD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNTMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNTMD_A::_1
    }
}
#[doc = "Field `CNTMD` writer - Continuous Transfer Mode"]
pub type CNTMD_W<'a, const O: u8> = crate::BitWriter<'a, u16, DCPCFG_SPEC, CNTMD_A, O>;
impl<'a, const O: u8> CNTMD_W<'a, O> {
    #[doc = "Non-continuous transfer mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTMD_A::_0)
    }
    #[doc = "Continuous transfer mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTMD_A::_1)
    }
}
impl R {
    #[doc = "Bit 4 - Transfer Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Pipe Blocking on End of Transfer"]
    #[inline(always)]
    pub fn shtnak(&self) -> SHTNAK_R {
        SHTNAK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Continuous Transfer Mode"]
    #[inline(always)]
    pub fn cntmd(&self) -> CNTMD_R {
        CNTMD_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Transfer Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<4> {
        DIR_W::new(self)
    }
    #[doc = "Bit 7 - Pipe Blocking on End of Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn shtnak(&mut self) -> SHTNAK_W<7> {
        SHTNAK_W::new(self)
    }
    #[doc = "Bit 8 - Continuous Transfer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cntmd(&mut self) -> CNTMD_W<8> {
        CNTMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCP Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcpcfg](index.html) module"]
pub struct DCPCFG_SPEC;
impl crate::RegisterSpec for DCPCFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dcpcfg::R](R) reader structure"]
impl crate::Readable for DCPCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcpcfg::W](W) writer structure"]
impl crate::Writable for DCPCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCPCFG to value 0"]
impl crate::Resettable for DCPCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
