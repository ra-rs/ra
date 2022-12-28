#[doc = "Register `SRAMWTSC` reader"]
pub struct R(crate::R<SRAMWTSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAMWTSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAMWTSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAMWTSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAMWTSC` writer"]
pub struct W(crate::W<SRAMWTSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAMWTSC_SPEC>;
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
impl From<crate::W<SRAMWTSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAMWTSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM0WTEN` reader - SRAM0 wait enable"]
pub type SRAM0WTEN_R = crate::BitReader<SRAM0WTEN_A>;
#[doc = "SRAM0 wait enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM0WTEN_A {
    #[doc = "0: No wait"]
    _0 = 0,
    #[doc = "1: Add wait state in read access cycle to SRAM0"]
    _1 = 1,
}
impl From<SRAM0WTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM0WTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM0WTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM0WTEN_A {
        match self.bits {
            false => SRAM0WTEN_A::_0,
            true => SRAM0WTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRAM0WTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRAM0WTEN_A::_1
    }
}
#[doc = "Field `SRAM0WTEN` writer - SRAM0 wait enable"]
pub type SRAM0WTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SRAMWTSC_SPEC, SRAM0WTEN_A, O>;
impl<'a, const O: u8> SRAM0WTEN_W<'a, O> {
    #[doc = "No wait"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRAM0WTEN_A::_0)
    }
    #[doc = "Add wait state in read access cycle to SRAM0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRAM0WTEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - SRAM0 wait enable"]
    #[inline(always)]
    pub fn sram0wten(&self) -> SRAM0WTEN_R {
        SRAM0WTEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM0 wait enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram0wten(&mut self) -> SRAM0WTEN_W<0> {
        SRAM0WTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Wait State Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sramwtsc](index.html) module"]
pub struct SRAMWTSC_SPEC;
impl crate::RegisterSpec for SRAMWTSC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sramwtsc::R](R) reader structure"]
impl crate::Readable for SRAMWTSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sramwtsc::W](W) writer structure"]
impl crate::Writable for SRAMWTSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAMWTSC to value 0x01"]
impl crate::Resettable for SRAMWTSC_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
