#[doc = "Register `SDCMOD` reader"]
pub struct R(crate::R<SDCMOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDCMOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDCMOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDCMOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDCMOD` writer"]
pub struct W(crate::W<SDCMOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDCMOD_SPEC>;
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
impl From<crate::W<SDCMOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDCMOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMODE` reader - Endian Mode"]
pub type EMODE_R = crate::BitReader<EMODE_A>;
#[doc = "Endian Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMODE_A {
    #[doc = "0: Endian order of SDRAM address space is the same as the endian order of the operating mode"]
    _0 = 0,
    #[doc = "1: Endian order of SDRAM address space is not the endian order of the operating mode."]
    _1 = 1,
}
impl From<EMODE_A> for bool {
    #[inline(always)]
    fn from(variant: EMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl EMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMODE_A {
        match self.bits {
            false => EMODE_A::_0,
            true => EMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EMODE_A::_1
    }
}
#[doc = "Field `EMODE` writer - Endian Mode"]
pub type EMODE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SDCMOD_SPEC, EMODE_A, O>;
impl<'a, const O: u8> EMODE_W<'a, O> {
    #[doc = "Endian order of SDRAM address space is the same as the endian order of the operating mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EMODE_A::_0)
    }
    #[doc = "Endian order of SDRAM address space is not the endian order of the operating mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EMODE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Endian Mode"]
    #[inline(always)]
    pub fn emode(&self) -> EMODE_R {
        EMODE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endian Mode"]
    #[inline(always)]
    #[must_use]
    pub fn emode(&mut self) -> EMODE_W<0> {
        EMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDC Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdcmod](index.html) module"]
pub struct SDCMOD_SPEC;
impl crate::RegisterSpec for SDCMOD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sdcmod::R](R) reader structure"]
impl crate::Readable for SDCMOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdcmod::W](W) writer structure"]
impl crate::Writable for SDCMOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDCMOD to value 0"]
impl crate::Resettable for SDCMOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
