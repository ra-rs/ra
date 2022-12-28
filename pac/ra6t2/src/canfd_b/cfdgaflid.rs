#[doc = "Register `CFDGAFLID%s` reader"]
pub struct R(crate::R<CFDGAFLID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDGAFLID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDGAFLID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDGAFLID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDGAFLID%s` writer"]
pub struct W(crate::W<CFDGAFLID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDGAFLID_SPEC>;
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
impl From<crate::W<CFDGAFLID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDGAFLID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GAFLID` reader - Global Acceptance Filter List Entry ID Field"]
pub type GAFLID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GAFLID` writer - Global Acceptance Filter List Entry ID Field"]
pub type GAFLID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFDGAFLID_SPEC, u32, u32, 29, O>;
#[doc = "Field `GAFLLB` reader - Global Acceptance Filter List Entry Loopback Configuration"]
pub type GAFLLB_R = crate::BitReader<GAFLLB_A>;
#[doc = "Global Acceptance Filter List Entry Loopback Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GAFLLB_A {
    #[doc = "0: Global Acceptance Filter List entry ID for acceptance filtering with attribute RX"]
    _0 = 0,
    #[doc = "1: Global Acceptance Filter List entry ID for acceptance filtering with attribute TX"]
    _1 = 1,
}
impl From<GAFLLB_A> for bool {
    #[inline(always)]
    fn from(variant: GAFLLB_A) -> Self {
        variant as u8 != 0
    }
}
impl GAFLLB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAFLLB_A {
        match self.bits {
            false => GAFLLB_A::_0,
            true => GAFLLB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GAFLLB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GAFLLB_A::_1
    }
}
#[doc = "Field `GAFLLB` writer - Global Acceptance Filter List Entry Loopback Configuration"]
pub type GAFLLB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGAFLID_SPEC, GAFLLB_A, O>;
impl<'a, const O: u8> GAFLLB_W<'a, O> {
    #[doc = "Global Acceptance Filter List entry ID for acceptance filtering with attribute RX"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GAFLLB_A::_0)
    }
    #[doc = "Global Acceptance Filter List entry ID for acceptance filtering with attribute TX"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GAFLLB_A::_1)
    }
}
#[doc = "Field `GAFLRTR` reader - Global Acceptance Filter List Entry RTR Field"]
pub type GAFLRTR_R = crate::BitReader<GAFLRTR_A>;
#[doc = "Global Acceptance Filter List Entry RTR Field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GAFLRTR_A {
    #[doc = "0: Data frame"]
    _0 = 0,
    #[doc = "1: Remote frame"]
    _1 = 1,
}
impl From<GAFLRTR_A> for bool {
    #[inline(always)]
    fn from(variant: GAFLRTR_A) -> Self {
        variant as u8 != 0
    }
}
impl GAFLRTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAFLRTR_A {
        match self.bits {
            false => GAFLRTR_A::_0,
            true => GAFLRTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GAFLRTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GAFLRTR_A::_1
    }
}
#[doc = "Field `GAFLRTR` writer - Global Acceptance Filter List Entry RTR Field"]
pub type GAFLRTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGAFLID_SPEC, GAFLRTR_A, O>;
impl<'a, const O: u8> GAFLRTR_W<'a, O> {
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GAFLRTR_A::_0)
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GAFLRTR_A::_1)
    }
}
#[doc = "Field `GAFLIDE` reader - Global Acceptance Filter List Entry IDE Field"]
pub type GAFLIDE_R = crate::BitReader<GAFLIDE_A>;
#[doc = "Global Acceptance Filter List Entry IDE Field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GAFLIDE_A {
    #[doc = "0: Standard identifier of rule entry ID is valid for acceptance filtering"]
    _0 = 0,
    #[doc = "1: Extended identifier of rule entry ID is valid for acceptance filtering"]
    _1 = 1,
}
impl From<GAFLIDE_A> for bool {
    #[inline(always)]
    fn from(variant: GAFLIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl GAFLIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAFLIDE_A {
        match self.bits {
            false => GAFLIDE_A::_0,
            true => GAFLIDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GAFLIDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GAFLIDE_A::_1
    }
}
#[doc = "Field `GAFLIDE` writer - Global Acceptance Filter List Entry IDE Field"]
pub type GAFLIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDGAFLID_SPEC, GAFLIDE_A, O>;
impl<'a, const O: u8> GAFLIDE_W<'a, O> {
    #[doc = "Standard identifier of rule entry ID is valid for acceptance filtering"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GAFLIDE_A::_0)
    }
    #[doc = "Extended identifier of rule entry ID is valid for acceptance filtering"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GAFLIDE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:28 - Global Acceptance Filter List Entry ID Field"]
    #[inline(always)]
    pub fn gaflid(&self) -> GAFLID_R {
        GAFLID_R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - Global Acceptance Filter List Entry Loopback Configuration"]
    #[inline(always)]
    pub fn gafllb(&self) -> GAFLLB_R {
        GAFLLB_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Global Acceptance Filter List Entry RTR Field"]
    #[inline(always)]
    pub fn gaflrtr(&self) -> GAFLRTR_R {
        GAFLRTR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Global Acceptance Filter List Entry IDE Field"]
    #[inline(always)]
    pub fn gaflide(&self) -> GAFLIDE_R {
        GAFLIDE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Global Acceptance Filter List Entry ID Field"]
    #[inline(always)]
    #[must_use]
    pub fn gaflid(&mut self) -> GAFLID_W<0> {
        GAFLID_W::new(self)
    }
    #[doc = "Bit 29 - Global Acceptance Filter List Entry Loopback Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn gafllb(&mut self) -> GAFLLB_W<29> {
        GAFLLB_W::new(self)
    }
    #[doc = "Bit 30 - Global Acceptance Filter List Entry RTR Field"]
    #[inline(always)]
    #[must_use]
    pub fn gaflrtr(&mut self) -> GAFLRTR_W<30> {
        GAFLRTR_W::new(self)
    }
    #[doc = "Bit 31 - Global Acceptance Filter List Entry IDE Field"]
    #[inline(always)]
    #[must_use]
    pub fn gaflide(&mut self) -> GAFLIDE_W<31> {
        GAFLIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Acceptance Filter List ID Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdgaflid](index.html) module"]
pub struct CFDGAFLID_SPEC;
impl crate::RegisterSpec for CFDGAFLID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdgaflid::R](R) reader structure"]
impl crate::Readable for CFDGAFLID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdgaflid::W](W) writer structure"]
impl crate::Writable for CFDGAFLID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDGAFLID%s to value 0"]
impl crate::Resettable for CFDGAFLID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
