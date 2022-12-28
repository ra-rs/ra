#[doc = "Register `CRCSAR` reader"]
pub struct R(crate::R<CRCSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCSAR` writer"]
pub struct W(crate::W<CRCSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCSAR_SPEC>;
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
impl From<crate::W<CRCSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCSA` reader - snoop address bitSet the I/O register address to snoop"]
pub type CRCSA_R = crate::FieldReader<u16, CRCSA_A>;
#[doc = "snoop address bitSet the I/O register address to snoop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum CRCSA_A {
    #[doc = "3: SCI0.TDR"]
    _0X0003 = 3,
    #[doc = "5: SCI0.RDR"]
    _0X0005 = 5,
    #[doc = "35: SCI1.TDR"]
    _0X0023 = 35,
    #[doc = "37: SCI1.RDR"]
    _0X0025 = 37,
    #[doc = "67: SCI2.TDR"]
    _0X0043 = 67,
    #[doc = "69: SCI2.RDR"]
    _0X0045 = 69,
    #[doc = "99: SCI3.TDR"]
    _0X0063 = 99,
    #[doc = "101: SCI3.RDR"]
    _0X0065 = 101,
    #[doc = "131: SCI4.TDR"]
    _0X0083 = 131,
    #[doc = "133: SCI4.RDR"]
    _0X0085 = 133,
    #[doc = "291: SCI9.TDR"]
    _0X0123 = 291,
    #[doc = "293: SCI9.RDR"]
    _0X0125 = 293,
}
impl From<CRCSA_A> for u16 {
    #[inline(always)]
    fn from(variant: CRCSA_A) -> Self {
        variant as _
    }
}
impl CRCSA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRCSA_A> {
        match self.bits {
            3 => Some(CRCSA_A::_0X0003),
            5 => Some(CRCSA_A::_0X0005),
            35 => Some(CRCSA_A::_0X0023),
            37 => Some(CRCSA_A::_0X0025),
            67 => Some(CRCSA_A::_0X0043),
            69 => Some(CRCSA_A::_0X0045),
            99 => Some(CRCSA_A::_0X0063),
            101 => Some(CRCSA_A::_0X0065),
            131 => Some(CRCSA_A::_0X0083),
            133 => Some(CRCSA_A::_0X0085),
            291 => Some(CRCSA_A::_0X0123),
            293 => Some(CRCSA_A::_0X0125),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0003`"]
    #[inline(always)]
    pub fn is_0x0003(&self) -> bool {
        *self == CRCSA_A::_0X0003
    }
    #[doc = "Checks if the value of the field is `_0X0005`"]
    #[inline(always)]
    pub fn is_0x0005(&self) -> bool {
        *self == CRCSA_A::_0X0005
    }
    #[doc = "Checks if the value of the field is `_0X0023`"]
    #[inline(always)]
    pub fn is_0x0023(&self) -> bool {
        *self == CRCSA_A::_0X0023
    }
    #[doc = "Checks if the value of the field is `_0X0025`"]
    #[inline(always)]
    pub fn is_0x0025(&self) -> bool {
        *self == CRCSA_A::_0X0025
    }
    #[doc = "Checks if the value of the field is `_0X0043`"]
    #[inline(always)]
    pub fn is_0x0043(&self) -> bool {
        *self == CRCSA_A::_0X0043
    }
    #[doc = "Checks if the value of the field is `_0X0045`"]
    #[inline(always)]
    pub fn is_0x0045(&self) -> bool {
        *self == CRCSA_A::_0X0045
    }
    #[doc = "Checks if the value of the field is `_0X0063`"]
    #[inline(always)]
    pub fn is_0x0063(&self) -> bool {
        *self == CRCSA_A::_0X0063
    }
    #[doc = "Checks if the value of the field is `_0X0065`"]
    #[inline(always)]
    pub fn is_0x0065(&self) -> bool {
        *self == CRCSA_A::_0X0065
    }
    #[doc = "Checks if the value of the field is `_0X0083`"]
    #[inline(always)]
    pub fn is_0x0083(&self) -> bool {
        *self == CRCSA_A::_0X0083
    }
    #[doc = "Checks if the value of the field is `_0X0085`"]
    #[inline(always)]
    pub fn is_0x0085(&self) -> bool {
        *self == CRCSA_A::_0X0085
    }
    #[doc = "Checks if the value of the field is `_0X0123`"]
    #[inline(always)]
    pub fn is_0x0123(&self) -> bool {
        *self == CRCSA_A::_0X0123
    }
    #[doc = "Checks if the value of the field is `_0X0125`"]
    #[inline(always)]
    pub fn is_0x0125(&self) -> bool {
        *self == CRCSA_A::_0X0125
    }
}
#[doc = "Field `CRCSA` writer - snoop address bitSet the I/O register address to snoop"]
pub type CRCSA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CRCSAR_SPEC, u16, CRCSA_A, 14, O>;
impl<'a, const O: u8> CRCSA_W<'a, O> {
    #[doc = "SCI0.TDR"]
    #[inline(always)]
    pub fn _0x0003(self) -> &'a mut W {
        self.variant(CRCSA_A::_0X0003)
    }
    #[doc = "SCI0.RDR"]
    #[inline(always)]
    pub fn _0x0005(self) -> &'a mut W {
        self.variant(CRCSA_A::_0X0005)
    }
    #[doc = "SCI1.TDR"]
    #[inline(always)]
    pub fn _0x0023(self) -> &'a mut W {
        self.variant(CRCSA_A::_0X0023)
    }
    #[doc = "SCI1.RDR"]
    #[inline(always)]
    pub fn _0x0025(self) -> &'a mut W {
        self.variant(CRCSA_A::_0X0025)
    }
    #[doc = "SCI2.TDR"]
    #[inline(always)]
    pub fn _0x0043(self) -> &'a mut W {
        self.variant(CRCSA_A::_0X0043)
    }
    #[doc = "SCI2.RDR"]
    #[inline(always)]
    pub fn _0x0045(self) -> &'a mut W {
        self.variant(CRCSA_A::_0X0045)
    }
    #[doc = "SCI3.TDR"]
    #[inline(always)]
    pub fn _0x0063(self) -> &'a mut W {
        self.variant(CRCSA_A::_0X0063)
    }
    #[doc = "SCI3.RDR"]
    #[inline(always)]
    pub fn _0x0065(self) -> &'a mut W {
        self.variant(CRCSA_A::_0X0065)
    }
    #[doc = "SCI4.TDR"]
    #[inline(always)]
    pub fn _0x0083(self) -> &'a mut W {
        self.variant(CRCSA_A::_0X0083)
    }
    #[doc = "SCI4.RDR"]
    #[inline(always)]
    pub fn _0x0085(self) -> &'a mut W {
        self.variant(CRCSA_A::_0X0085)
    }
    #[doc = "SCI9.TDR"]
    #[inline(always)]
    pub fn _0x0123(self) -> &'a mut W {
        self.variant(CRCSA_A::_0X0123)
    }
    #[doc = "SCI9.RDR"]
    #[inline(always)]
    pub fn _0x0125(self) -> &'a mut W {
        self.variant(CRCSA_A::_0X0125)
    }
}
impl R {
    #[doc = "Bits 0:13 - snoop address bitSet the I/O register address to snoop"]
    #[inline(always)]
    pub fn crcsa(&self) -> CRCSA_R {
        CRCSA_R::new(self.bits & 0x3fff)
    }
}
impl W {
    #[doc = "Bits 0:13 - snoop address bitSet the I/O register address to snoop"]
    #[inline(always)]
    #[must_use]
    pub fn crcsa(&mut self) -> CRCSA_W<0> {
        CRCSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Snoop Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcsar](index.html) module"]
pub struct CRCSAR_SPEC;
impl crate::RegisterSpec for CRCSAR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crcsar::R](R) reader structure"]
impl crate::Readable for CRCSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcsar::W](W) writer structure"]
impl crate::Writable for CRCSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCSAR to value 0"]
impl crate::Resettable for CRCSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
