#[doc = "Register `GTDLYR%sB` reader"]
pub struct R(crate::R<GTDLYRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTDLYRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTDLYRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTDLYRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTDLYR%sB` writer"]
pub struct W(crate::W<GTDLYRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTDLYRB_SPEC>;
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
impl From<crate::W<GTDLYRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTDLYRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLY` reader - GTIOCnB Output Rising Edge Delay Setting"]
pub type DLY_R = crate::FieldReader<u8, DLY_A>;
#[doc = "GTIOCnB Output Rising Edge Delay Setting\n\nValue on reset: 0"]
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
#[doc = "Field `DLY` writer - GTIOCnB Output Rising Edge Delay Setting"]
pub type DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, GTDLYRB_SPEC, u8, DLY_A, 5, O>;
impl<'a, const O: u8> DLY_W<'a, O> {
    #[doc = "No delay on rising edges"]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut W {
        self.variant(DLY_A::_00000)
    }
}
impl R {
    #[doc = "Bits 0:4 - GTIOCnB Output Rising Edge Delay Setting"]
    #[inline(always)]
    pub fn dly(&self) -> DLY_R {
        DLY_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - GTIOCnB Output Rising Edge Delay Setting"]
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
#[doc = "GTIOC%sB Rising Output Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtdlyrb](index.html) module"]
pub struct GTDLYRB_SPEC;
impl crate::RegisterSpec for GTDLYRB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gtdlyrb::R](R) reader structure"]
impl crate::Readable for GTDLYRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtdlyrb::W](W) writer structure"]
impl crate::Writable for GTDLYRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTDLYR%sB to value 0"]
impl crate::Resettable for GTDLYRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
