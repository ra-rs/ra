#[doc = "Register `SPCR3` reader"]
pub struct R(crate::R<SPCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPCR3` writer"]
pub struct W(crate::W<SPCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPCR3_SPEC>;
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
impl From<crate::W<SPCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSL0P` reader - SSL0 Signal Polarity"]
pub type SSL0P_R = crate::BitReader<SSL0P_A>;
#[doc = "SSL0 Signal Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSL0P_A {
    #[doc = "0: The SSL0 signal is active low (0)."]
    _0 = 0,
    #[doc = "1: The SSL0 signal is active high (1)."]
    _1 = 1,
}
impl From<SSL0P_A> for bool {
    #[inline(always)]
    fn from(variant: SSL0P_A) -> Self {
        variant as u8 != 0
    }
}
impl SSL0P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSL0P_A {
        match self.bits {
            false => SSL0P_A::_0,
            true => SSL0P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSL0P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSL0P_A::_1
    }
}
#[doc = "Field `SSL0P` writer - SSL0 Signal Polarity"]
pub type SSL0P_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR3_SPEC, SSL0P_A, O>;
impl<'a, const O: u8> SSL0P_W<'a, O> {
    #[doc = "The SSL0 signal is active low (0)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSL0P_A::_0)
    }
    #[doc = "The SSL0 signal is active high (1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSL0P_A::_1)
    }
}
#[doc = "Field `SSL1P` reader - SSL1 Signal Polarity"]
pub type SSL1P_R = crate::BitReader<SSL1P_A>;
#[doc = "SSL1 Signal Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSL1P_A {
    #[doc = "0: The SSL1 signal is active low (0)."]
    _0 = 0,
    #[doc = "1: The SSL1 signal is active high (1)."]
    _1 = 1,
}
impl From<SSL1P_A> for bool {
    #[inline(always)]
    fn from(variant: SSL1P_A) -> Self {
        variant as u8 != 0
    }
}
impl SSL1P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSL1P_A {
        match self.bits {
            false => SSL1P_A::_0,
            true => SSL1P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSL1P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSL1P_A::_1
    }
}
#[doc = "Field `SSL1P` writer - SSL1 Signal Polarity"]
pub type SSL1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR3_SPEC, SSL1P_A, O>;
impl<'a, const O: u8> SSL1P_W<'a, O> {
    #[doc = "The SSL1 signal is active low (0)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSL1P_A::_0)
    }
    #[doc = "The SSL1 signal is active high (1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSL1P_A::_1)
    }
}
#[doc = "Field `SSL2P` reader - SSL2 Signal Polarity"]
pub type SSL2P_R = crate::BitReader<SSL2P_A>;
#[doc = "SSL2 Signal Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSL2P_A {
    #[doc = "0: The SSL2 signal is active low (0)."]
    _0 = 0,
    #[doc = "1: The SSL2 signal is active high (1)."]
    _1 = 1,
}
impl From<SSL2P_A> for bool {
    #[inline(always)]
    fn from(variant: SSL2P_A) -> Self {
        variant as u8 != 0
    }
}
impl SSL2P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSL2P_A {
        match self.bits {
            false => SSL2P_A::_0,
            true => SSL2P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSL2P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSL2P_A::_1
    }
}
#[doc = "Field `SSL2P` writer - SSL2 Signal Polarity"]
pub type SSL2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR3_SPEC, SSL2P_A, O>;
impl<'a, const O: u8> SSL2P_W<'a, O> {
    #[doc = "The SSL2 signal is active low (0)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSL2P_A::_0)
    }
    #[doc = "The SSL2 signal is active high (1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSL2P_A::_1)
    }
}
#[doc = "Field `SSL3P` reader - SSL3 Signal Polarity"]
pub type SSL3P_R = crate::BitReader<SSL3P_A>;
#[doc = "SSL3 Signal Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSL3P_A {
    #[doc = "0: The SSL3 signal is active low (0)."]
    _0 = 0,
    #[doc = "1: The SSL3 signal is active high (1)."]
    _1 = 1,
}
impl From<SSL3P_A> for bool {
    #[inline(always)]
    fn from(variant: SSL3P_A) -> Self {
        variant as u8 != 0
    }
}
impl SSL3P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSL3P_A {
        match self.bits {
            false => SSL3P_A::_0,
            true => SSL3P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSL3P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSL3P_A::_1
    }
}
#[doc = "Field `SSL3P` writer - SSL3 Signal Polarity"]
pub type SSL3P_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPCR3_SPEC, SSL3P_A, O>;
impl<'a, const O: u8> SSL3P_W<'a, O> {
    #[doc = "The SSL3 signal is active low (0)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSL3P_A::_0)
    }
    #[doc = "The SSL3 signal is active high (1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSL3P_A::_1)
    }
}
#[doc = "Field `SPBR` reader - SPI Bit Rate"]
pub type SPBR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPBR` writer - SPI Bit Rate"]
pub type SPBR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPCR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `SPSLN` reader - SPI Sequence Length"]
pub type SPSLN_R = crate::FieldReader<u8, SPSLN_A>;
#[doc = "SPI Sequence Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPSLN_A {
    #[doc = "0: Sequence Length is 1 (Referenced SPCMDn, n = 0â\u{86}\u{92}0â\u{86}\u{92}â\u{80}¦)"]
    _000 = 0,
    #[doc = "1: Sequence Length is 2 (Referenced SPCMDn, n = 0â\u{86}\u{92}1â\u{86}\u{92}0â\u{86}\u{92}â\u{80}¦)"]
    _001 = 1,
    #[doc = "2: Sequence Length is 3 (Referenced SPCMDn, n = 0â\u{86}\u{92}1â\u{86}\u{92}2â\u{86}\u{92}0â\u{86}\u{92}â\u{80}¦)"]
    _010 = 2,
    #[doc = "3: Sequence Length is 4 (Referenced SPCMDn, n = 0â\u{86}\u{92}1â\u{86}\u{92}2â\u{86}\u{92}3â\u{86}\u{92}0â\u{86}\u{92}â\u{80}¦)"]
    _011 = 3,
    #[doc = "4: Sequence Length is 5 (Referenced SPCMDn, n = 0â\u{86}\u{92}1â\u{86}\u{92}2â\u{86}\u{92}3â\u{86}\u{92}4â\u{86}\u{92}0â\u{86}\u{92}â\u{80}¦)"]
    _100 = 4,
    #[doc = "5: Sequence Length is 6 (Referenced SPCMDn, n = 0â\u{86}\u{92}1â\u{86}\u{92}2â\u{86}\u{92}3â\u{86}\u{92}4â\u{86}\u{92}5â\u{86}\u{92}0â\u{86}\u{92}â\u{80}¦)"]
    _101 = 5,
    #[doc = "6: Sequence Length is 7 (Referenced SPCMDn, n = 0â\u{86}\u{92}1â\u{86}\u{92}2â\u{86}\u{92}3â\u{86}\u{92}4â\u{86}\u{92}5â\u{86}\u{92}6â\u{86}\u{92}0â\u{86}\u{92}â\u{80}¦)"]
    _110 = 6,
    #[doc = "7: Sequence Length is 8 (Referenced SPCMDn, n = 0â\u{86}\u{92}1â\u{86}\u{92}2â\u{86}\u{92}3â\u{86}\u{92}4â\u{86}\u{92}5â\u{86}\u{92}6â\u{86}\u{92}7â\u{86}\u{92}0â\u{86}\u{92}â\u{80}¦)"]
    _111 = 7,
}
impl From<SPSLN_A> for u8 {
    #[inline(always)]
    fn from(variant: SPSLN_A) -> Self {
        variant as _
    }
}
impl SPSLN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPSLN_A {
        match self.bits {
            0 => SPSLN_A::_000,
            1 => SPSLN_A::_001,
            2 => SPSLN_A::_010,
            3 => SPSLN_A::_011,
            4 => SPSLN_A::_100,
            5 => SPSLN_A::_101,
            6 => SPSLN_A::_110,
            7 => SPSLN_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SPSLN_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SPSLN_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SPSLN_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SPSLN_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SPSLN_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SPSLN_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SPSLN_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SPSLN_A::_111
    }
}
#[doc = "Field `SPSLN` writer - SPI Sequence Length"]
pub type SPSLN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SPCR3_SPEC, u8, SPSLN_A, 3, O>;
impl<'a, const O: u8> SPSLN_W<'a, O> {
    #[doc = "Sequence Length is 1 (Referenced SPCMDn, n = 0â\u{86}\u{92}0â\u{86}\u{92}â\u{80}¦)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SPSLN_A::_000)
    }
    #[doc = "Sequence Length is 2 (Referenced SPCMDn, n = 0â\u{86}\u{92}1â\u{86}\u{92}0â\u{86}\u{92}â\u{80}¦)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SPSLN_A::_001)
    }
    #[doc = "Sequence Length is 3 (Referenced SPCMDn, n = 0â\u{86}\u{92}1â\u{86}\u{92}2â\u{86}\u{92}0â\u{86}\u{92}â\u{80}¦)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SPSLN_A::_010)
    }
    #[doc = "Sequence Length is 4 (Referenced SPCMDn, n = 0â\u{86}\u{92}1â\u{86}\u{92}2â\u{86}\u{92}3â\u{86}\u{92}0â\u{86}\u{92}â\u{80}¦)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SPSLN_A::_011)
    }
    #[doc = "Sequence Length is 5 (Referenced SPCMDn, n = 0â\u{86}\u{92}1â\u{86}\u{92}2â\u{86}\u{92}3â\u{86}\u{92}4â\u{86}\u{92}0â\u{86}\u{92}â\u{80}¦)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SPSLN_A::_100)
    }
    #[doc = "Sequence Length is 6 (Referenced SPCMDn, n = 0â\u{86}\u{92}1â\u{86}\u{92}2â\u{86}\u{92}3â\u{86}\u{92}4â\u{86}\u{92}5â\u{86}\u{92}0â\u{86}\u{92}â\u{80}¦)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SPSLN_A::_101)
    }
    #[doc = "Sequence Length is 7 (Referenced SPCMDn, n = 0â\u{86}\u{92}1â\u{86}\u{92}2â\u{86}\u{92}3â\u{86}\u{92}4â\u{86}\u{92}5â\u{86}\u{92}6â\u{86}\u{92}0â\u{86}\u{92}â\u{80}¦)"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SPSLN_A::_110)
    }
    #[doc = "Sequence Length is 8 (Referenced SPCMDn, n = 0â\u{86}\u{92}1â\u{86}\u{92}2â\u{86}\u{92}3â\u{86}\u{92}4â\u{86}\u{92}5â\u{86}\u{92}6â\u{86}\u{92}7â\u{86}\u{92}0â\u{86}\u{92}â\u{80}¦)"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(SPSLN_A::_111)
    }
}
impl R {
    #[doc = "Bit 0 - SSL0 Signal Polarity"]
    #[inline(always)]
    pub fn ssl0p(&self) -> SSL0P_R {
        SSL0P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSL1 Signal Polarity"]
    #[inline(always)]
    pub fn ssl1p(&self) -> SSL1P_R {
        SSL1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSL2 Signal Polarity"]
    #[inline(always)]
    pub fn ssl2p(&self) -> SSL2P_R {
        SSL2P_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSL3 Signal Polarity"]
    #[inline(always)]
    pub fn ssl3p(&self) -> SSL3P_R {
        SSL3P_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15 - SPI Bit Rate"]
    #[inline(always)]
    pub fn spbr(&self) -> SPBR_R {
        SPBR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - SPI Sequence Length"]
    #[inline(always)]
    pub fn spsln(&self) -> SPSLN_R {
        SPSLN_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SSL0 Signal Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ssl0p(&mut self) -> SSL0P_W<0> {
        SSL0P_W::new(self)
    }
    #[doc = "Bit 1 - SSL1 Signal Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ssl1p(&mut self) -> SSL1P_W<1> {
        SSL1P_W::new(self)
    }
    #[doc = "Bit 2 - SSL2 Signal Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ssl2p(&mut self) -> SSL2P_W<2> {
        SSL2P_W::new(self)
    }
    #[doc = "Bit 3 - SSL3 Signal Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ssl3p(&mut self) -> SSL3P_W<3> {
        SSL3P_W::new(self)
    }
    #[doc = "Bits 8:15 - SPI Bit Rate"]
    #[inline(always)]
    #[must_use]
    pub fn spbr(&mut self) -> SPBR_W<8> {
        SPBR_W::new(self)
    }
    #[doc = "Bits 24:26 - SPI Sequence Length"]
    #[inline(always)]
    #[must_use]
    pub fn spsln(&mut self) -> SPSLN_W<24> {
        SPSLN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spcr3](index.html) module"]
pub struct SPCR3_SPEC;
impl crate::RegisterSpec for SPCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spcr3::R](R) reader structure"]
impl crate::Readable for SPCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spcr3::W](W) writer structure"]
impl crate::Writable for SPCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPCR3 to value 0xff00"]
impl crate::Resettable for SPCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
