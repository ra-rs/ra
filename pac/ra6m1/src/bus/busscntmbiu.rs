#[doc = "Register `BUSSCNTMBIU` reader"]
pub struct R(crate::R<BUSSCNTMBIU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSSCNTMBIU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSSCNTMBIU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSSCNTMBIU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUSSCNTMBIU` writer"]
pub struct W(crate::W<BUSSCNTMBIU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUSSCNTMBIU_SPEC>;
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
impl From<crate::W<BUSSCNTMBIU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUSSCNTMBIU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARBMET` reader - Arbitration Method Specify the priority between groups"]
pub type ARBMET_R = crate::FieldReader<u8, ARBMET_A>;
#[doc = "Arbitration Method Specify the priority between groups\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARBMET_A {
    #[doc = "0: fixed priority"]
    _00 = 0,
    #[doc = "1: round-robin"]
    _01 = 1,
}
impl From<ARBMET_A> for u8 {
    #[inline(always)]
    fn from(variant: ARBMET_A) -> Self {
        variant as _
    }
}
impl ARBMET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ARBMET_A> {
        match self.bits {
            0 => Some(ARBMET_A::_00),
            1 => Some(ARBMET_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ARBMET_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ARBMET_A::_01
    }
}
#[doc = "Field `ARBMET` writer - Arbitration Method Specify the priority between groups"]
pub type ARBMET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, BUSSCNTMBIU_SPEC, u8, ARBMET_A, 2, O>;
impl<'a, const O: u8> ARBMET_W<'a, O> {
    #[doc = "fixed priority"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ARBMET_A::_00)
    }
    #[doc = "round-robin"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ARBMET_A::_01)
    }
}
#[doc = "Field `EWRES` reader - Early Write Response Whether the next write request is accepted or not until a response for the write transaction comes back."]
pub type EWRES_R = crate::BitReader<EWRES_A>;
#[doc = "Early Write Response Whether the next write request is accepted or not until a response for the write transaction comes back.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWRES_A {
    #[doc = "0: Not accepted."]
    _0 = 0,
    #[doc = "1: Accepted but error response is ignored."]
    _1 = 1,
}
impl From<EWRES_A> for bool {
    #[inline(always)]
    fn from(variant: EWRES_A) -> Self {
        variant as u8 != 0
    }
}
impl EWRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWRES_A {
        match self.bits {
            false => EWRES_A::_0,
            true => EWRES_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EWRES_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EWRES_A::_1
    }
}
#[doc = "Field `EWRES` writer - Early Write Response Whether the next write request is accepted or not until a response for the write transaction comes back."]
pub type EWRES_W<'a, const O: u8> = crate::BitWriter<'a, u16, BUSSCNTMBIU_SPEC, EWRES_A, O>;
impl<'a, const O: u8> EWRES_W<'a, O> {
    #[doc = "Not accepted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EWRES_A::_0)
    }
    #[doc = "Accepted but error response is ignored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EWRES_A::_1)
    }
}
impl R {
    #[doc = "Bits 4:5 - Arbitration Method Specify the priority between groups"]
    #[inline(always)]
    pub fn arbmet(&self) -> ARBMET_R {
        ARBMET_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Early Write Response Whether the next write request is accepted or not until a response for the write transaction comes back."]
    #[inline(always)]
    pub fn ewres(&self) -> EWRES_R {
        EWRES_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5 - Arbitration Method Specify the priority between groups"]
    #[inline(always)]
    #[must_use]
    pub fn arbmet(&mut self) -> ARBMET_W<4> {
        ARBMET_W::new(self)
    }
    #[doc = "Bit 8 - Early Write Response Whether the next write request is accepted or not until a response for the write transaction comes back."]
    #[inline(always)]
    #[must_use]
    pub fn ewres(&mut self) -> EWRES_W<8> {
        EWRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Bus Control Register MBIU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busscntmbiu](index.html) module"]
pub struct BUSSCNTMBIU_SPEC;
impl crate::RegisterSpec for BUSSCNTMBIU_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [busscntmbiu::R](R) reader structure"]
impl crate::Readable for BUSSCNTMBIU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [busscntmbiu::W](W) writer structure"]
impl crate::Writable for BUSSCNTMBIU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUSSCNTMBIU to value 0"]
impl crate::Resettable for BUSSCNTMBIU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
