#[doc = "Register `GTDLYR%sA` reader"]
pub struct R(crate::R<GTDLYRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTDLYRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTDLYRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTDLYRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTDLYR%sA` writer"]
pub struct W(crate::W<GTDLYRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTDLYRA_SPEC>;
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
impl From<crate::W<GTDLYRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTDLYRA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLY` reader - GTIOCnA Output Rising Edge Delay Setting"]
pub type DLY_R = crate::FieldReader<u8, DLY_A>;
#[doc = "GTIOCnA Output Rising Edge Delay Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DLY_A {
    #[doc = "0: No delay on rising edges"]
    _00000 = 0,
}
impl From<DLY_A> for u8 {
    #[inline(always)]
    fn from(variant: DLY_A) -> Self {
        variant as _
    }
}
impl DLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DLY_A> {
        match self.bits {
            0 => Some(DLY_A::_00000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == DLY_A::_00000
    }
}
#[doc = "Field `DLY` writer - GTIOCnA Output Rising Edge Delay Setting"]
pub type DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, GTDLYRA_SPEC, u8, DLY_A, 5, O>;
impl<'a, const O: u8> DLY_W<'a, O> {
    #[doc = "No delay on rising edges"]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut W {
        self.variant(DLY_A::_00000)
    }
}
impl R {
    #[doc = "Bits 0:4 - GTIOCnA Output Rising Edge Delay Setting"]
    #[inline(always)]
    pub fn dly(&self) -> DLY_R {
        DLY_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - GTIOCnA Output Rising Edge Delay Setting"]
    #[inline(always)]
    #[must_use]
    pub fn dly(&mut self) -> DLY_W<0> {
        DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTIOC%sA Rising Output Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtdlyra](index.html) module"]
pub struct GTDLYRA_SPEC;
impl crate::RegisterSpec for GTDLYRA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gtdlyra::R](R) reader structure"]
impl crate::Readable for GTDLYRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtdlyra::W](W) writer structure"]
impl crate::Writable for GTDLYRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTDLYR%sA to value 0"]
impl crate::Resettable for GTDLYRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
