#[doc = "Register `DEVADDA` reader"]
pub struct R(crate::R<DEVADDA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVADDA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVADDA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVADDA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVADDA` writer"]
pub struct W(crate::W<DEVADDA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVADDA_SPEC>;
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
impl From<crate::W<DEVADDA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVADDA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBSPD` reader - Transfer Speed of Communication Target Device"]
pub type USBSPD_R = crate::FieldReader<u8, USBSPD_A>;
#[doc = "Transfer Speed of Communication Target Device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USBSPD_A {
    #[doc = "0: DEVADDA is not used."]
    _00 = 0,
    #[doc = "1: Low speed"]
    _01 = 1,
    #[doc = "2: Full speed"]
    _10 = 2,
    #[doc = "3: High speed"]
    _11 = 3,
}
impl From<USBSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: USBSPD_A) -> Self {
        variant as _
    }
}
impl USBSPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBSPD_A {
        match self.bits {
            0 => USBSPD_A::_00,
            1 => USBSPD_A::_01,
            2 => USBSPD_A::_10,
            3 => USBSPD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == USBSPD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == USBSPD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == USBSPD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == USBSPD_A::_11
    }
}
#[doc = "Field `USBSPD` writer - Transfer Speed of Communication Target Device"]
pub type USBSPD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, DEVADDA_SPEC, u8, USBSPD_A, 2, O>;
impl<'a, const O: u8> USBSPD_W<'a, O> {
    #[doc = "DEVADDA is not used."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(USBSPD_A::_00)
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(USBSPD_A::_01)
    }
    #[doc = "Full speed"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(USBSPD_A::_10)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(USBSPD_A::_11)
    }
}
#[doc = "Field `HUBPORT` reader - Communication Target Connecting Hub Port"]
pub type HUBPORT_R = crate::FieldReader<u8, HUBPORT_A>;
#[doc = "Communication Target Connecting Hub Port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HUBPORT_A {
    #[doc = "0: Directly connected to the port of the USBHS."]
    _000 = 0,
}
impl From<HUBPORT_A> for u8 {
    #[inline(always)]
    fn from(variant: HUBPORT_A) -> Self {
        variant as _
    }
}
impl HUBPORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HUBPORT_A> {
        match self.bits {
            0 => Some(HUBPORT_A::_000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == HUBPORT_A::_000
    }
}
#[doc = "Field `HUBPORT` writer - Communication Target Connecting Hub Port"]
pub type HUBPORT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, DEVADDA_SPEC, u8, HUBPORT_A, 3, O>;
impl<'a, const O: u8> HUBPORT_W<'a, O> {
    #[doc = "Directly connected to the port of the USBHS."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(HUBPORT_A::_000)
    }
}
#[doc = "Field `UPPHUB` reader - Communication Target Connecting Hub Register"]
pub type UPPHUB_R = crate::FieldReader<u8, UPPHUB_A>;
#[doc = "Communication Target Connecting Hub Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UPPHUB_A {
    #[doc = "0: Directly connected to the port of the USBHS."]
    _0000 = 0,
}
impl From<UPPHUB_A> for u8 {
    #[inline(always)]
    fn from(variant: UPPHUB_A) -> Self {
        variant as _
    }
}
impl UPPHUB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UPPHUB_A> {
        match self.bits {
            0 => Some(UPPHUB_A::_0000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == UPPHUB_A::_0000
    }
}
#[doc = "Field `UPPHUB` writer - Communication Target Connecting Hub Register"]
pub type UPPHUB_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DEVADDA_SPEC, u8, UPPHUB_A, 4, O>;
impl<'a, const O: u8> UPPHUB_W<'a, O> {
    #[doc = "Directly connected to the port of the USBHS."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(UPPHUB_A::_0000)
    }
}
impl R {
    #[doc = "Bits 6:7 - Transfer Speed of Communication Target Device"]
    #[inline(always)]
    pub fn usbspd(&self) -> USBSPD_R {
        USBSPD_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Communication Target Connecting Hub Port"]
    #[inline(always)]
    pub fn hubport(&self) -> HUBPORT_R {
        HUBPORT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:14 - Communication Target Connecting Hub Register"]
    #[inline(always)]
    pub fn upphub(&self) -> UPPHUB_R {
        UPPHUB_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Transfer Speed of Communication Target Device"]
    #[inline(always)]
    #[must_use]
    pub fn usbspd(&mut self) -> USBSPD_W<6> {
        USBSPD_W::new(self)
    }
    #[doc = "Bits 8:10 - Communication Target Connecting Hub Port"]
    #[inline(always)]
    #[must_use]
    pub fn hubport(&mut self) -> HUBPORT_W<8> {
        HUBPORT_W::new(self)
    }
    #[doc = "Bits 11:14 - Communication Target Connecting Hub Register"]
    #[inline(always)]
    #[must_use]
    pub fn upphub(&mut self) -> UPPHUB_W<11> {
        UPPHUB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Address Configuration Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devadda](index.html) module"]
pub struct DEVADDA_SPEC;
impl crate::RegisterSpec for DEVADDA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [devadda::R](R) reader structure"]
impl crate::Readable for DEVADDA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devadda::W](W) writer structure"]
impl crate::Writable for DEVADDA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVADDA to value 0"]
impl crate::Resettable for DEVADDA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
